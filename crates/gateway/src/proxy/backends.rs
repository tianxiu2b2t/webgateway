use std::collections::VecDeque;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use rustls::{
    ClientConfig,
    pki_types::{DnsName, ServerName},
};
use shared::streams::WrapperBufferStream;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Semaphore};

// ---------- BackendConnection（只负责包装流，无状态）--------
#[derive(Debug)]
pub struct BackendConnection {
    inner: WrapperBufferStream,
}

impl BackendConnection {
    pub async fn new_tcp(addr: SocketAddr) -> anyhow::Result<Self> {
        Ok(Self {
            inner: WrapperBufferStream::Raw(TcpStream::connect(addr).await?),
        })
    }

    pub async fn new_tls(
        addr: SocketAddr,
        config: Arc<ClientConfig>,
        hostname: Option<impl Into<String>>,
    ) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Self::new_tls_from_raw(stream, config, hostname).await
    }

    pub async fn new_tls_from_raw(
        stream: TcpStream,
        config: Arc<ClientConfig>,
        hostname: Option<impl Into<String>>,
    ) -> anyhow::Result<Self> {
        let connector = tokio_rustls::TlsConnector::from(config);
        let server_name = match hostname {
            Some(h) => {
                let host = h.into();
                if let Ok(ip) = host.parse::<std::net::IpAddr>() {
                    ServerName::IpAddress(ip.into())
                } else {
                    DnsName::try_from(host)
                        .map_err(|_| anyhow::anyhow!("invalid DNS name"))?
                        .into()
                }
            }
            None => ServerName::IpAddress(stream.peer_addr()?.ip().into()),
        };
        Ok(Self {
            inner: WrapperBufferStream::TlsClient(Box::new(
                connector.connect(server_name, stream).await?,
            )),
        })
    }

    pub async fn close(self) -> anyhow::Result<()> {
        Ok(self.inner.close().await?)
    }

    /// 检查连接是否健康（这里简单示例，可自定义）
    pub async fn is_healthy(&mut self) -> bool {
        // 例如尝试一次零字节写（TLS 层会处理）
        matches!(
            tokio::time::timeout(std::time::Duration::from_millis(100), self.inner.write(&[]))
                .await,
            Ok(Ok(_))
        )
    }
}

// ---------- 正确的 AsyncRead/AsyncWrite 委托（修复递归）--------
impl AsyncRead for BackendConnection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for BackendConnection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write_vectored(cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

// ---------- 连接池配置 ----------
#[derive(Debug, Clone)]
pub struct BackendConnectionPoolConfig {
    pub target: SocketAddr,
    pub max_connections: usize, // 改为 usize，用 0 表示无限制
    pub tls: bool,
    pub tls_config: Option<Arc<ClientConfig>>,
    pub hostname: Option<String>,
}

impl BackendConnectionPoolConfig {
    pub fn new(target: SocketAddr) -> Self {
        Self {
            target,
            max_connections: 0,
            tls: false,
            tls_config: None,
            hostname: None,
        }
    }

    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    pub fn tls(mut self, config: Arc<ClientConfig>, hostname: Option<String>) -> Self {
        self.tls = true;
        self.tls_config = Some(config);
        self.hostname = hostname;
        self
    }
}

// ---------- 连接池核心 ----------
#[derive(Debug)]
pub struct BackendConnectionPool {
    config: BackendConnectionPoolConfig,
    idle: Mutex<VecDeque<BackendConnection>>, // 空闲连接队列
    semaphore: Arc<Semaphore>,                // 最大连接数信号量
}

impl BackendConnectionPool {
    pub fn new(config: BackendConnectionPoolConfig) -> Arc<Self> {
        let max = if config.max_connections == 0 {
            Semaphore::MAX_PERMITS
        } else {
            config.max_connections
        };
        Arc::new(Self {
            config,
            idle: Mutex::new(VecDeque::new()),
            semaphore: Arc::new(Semaphore::new(max)),
        })
    }

    /// 从池中获取一个连接（自动等待）
    pub async fn get(self: &Arc<Self>) -> anyhow::Result<PooledConnection> {
        // 获取 permit（信号量）
        let permit = self.semaphore.clone().acquire_owned().await?;

        // 循环尝试获取健康连接
        loop {
            // 1. 尝试从空闲队列取一个
            let mut idle = self.idle.lock().await;
            if let Some(mut conn) = idle.pop_front() {
                // 健康检查
                if conn.is_healthy().await {
                    return Ok(PooledConnection {
                        conn: Some(conn),
                        pool: self.clone(),
                        _permit: permit,
                    });
                } else {
                    // 不健康，关闭连接，释放 idle 锁，继续循环
                    drop(conn.close().await);
                    // 继续循环，再次尝试取下一个空闲连接
                    // 注意：permit 仍然有效，我们没有创建新连接，因此不会超过最大连接数
                    continue;
                }
            }
            // 没有空闲连接了，跳出循环去创建新连接
            break;
        }

        // 2. 创建新连接（此时没有空闲连接）
        let conn = if self.config.tls {
            let config = self.config.tls_config.clone().expect("TLS config missing");
            BackendConnection::new_tls(self.config.target, config, self.config.hostname.clone())
                .await?
        } else {
            BackendConnection::new_tcp(self.config.target).await?
        };

        Ok(PooledConnection {
            conn: Some(conn),
            pool: self.clone(),
            _permit: permit,
        })
    }

    /// 归还连接（内部方法）
    async fn return_connection(&self, mut conn: BackendConnection) {
        // 健康检查：不健康的连接直接关闭，不归还
        if !conn.is_healthy().await {
            let _ = conn.close().await;
            return;
        }

        // 尝试放回空闲队列
        let mut idle = self.idle.lock().await;
        // 可以设置一个 max_idle 限制，这里简单起见：总是放回
        idle.push_back(conn);
        // 注意：permit 此时已经被 drop，信号量计数已经归还
    }
}

// ---------- 借出连接句柄（自动归还）---------
pub struct PooledConnection {
    conn: Option<BackendConnection>, // Option 是为了能在 drop 时 move 出来
    pool: Arc<BackendConnectionPool>,
    _permit: tokio::sync::OwnedSemaphorePermit, // 持有 permit，离开作用域时自动释放
}

impl PooledConnection {
    /// 主动归还连接（一般不需要，Drop 会自动归还）
    pub async fn return_to_pool(mut self) -> anyhow::Result<()> {
        if let Some(conn) = self.conn.take() {
            self.pool.return_connection(conn).await;
        }
        Ok(())
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let pool = self.pool.clone();
            // 异步归还，避免在 drop 中阻塞
            tokio::spawn(async move {
                pool.return_connection(conn).await;
            });
        }
    }
}

// 委托 AsyncRead/AsyncWrite 给内部的 BackendConnection
impl AsyncRead for PooledConnection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(self.conn.as_mut().unwrap()).poll_read(cx, buf)
    }
}

impl AsyncWrite for PooledConnection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(self.conn.as_mut().unwrap()).poll_write(cx, buf)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(self.conn.as_mut().unwrap()).poll_write_vectored(cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.conn.as_ref().unwrap().is_write_vectored()
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(self.conn.as_mut().unwrap()).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(self.conn.as_mut().unwrap()).poll_shutdown(cx)
    }
}
