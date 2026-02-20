use std::{
    collections::HashMap,
    net::{SocketAddr, SocketAddrV4},
    sync::{Arc, LazyLock},
};

use hyper::{Request, body::Incoming, client, service::service_fn};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use shared::{
    default::sign_default_certificates,
    listener::CustomDualStackTcpListener,
    streams::{BufferStream, WrapperBufferStream},
};
use tokio::{net::TcpStream, sync::RwLock, task::JoinHandle};
use tokio_rustls::TlsAcceptor;
use tracing::{Level, event};

use crate::proxy::backends::{BackendConnectionPool, BackendConnectionPoolConfig};
pub mod backends;
pub mod protocols;

static HTTP_BUILDER: LazyLock<Builder<TokioExecutor>> = LazyLock::new(|| {
    hyper_util::server::conn::auto::Builder::<TokioExecutor>::new(TokioExecutor::new())
});

static LISTENERS: LazyLock<RwLock<HashMap<u16, JoinHandle<()>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static DEFAULT_TLS_CONFIG: LazyLock<Arc<TlsAcceptor>> =
    LazyLock::new(|| Arc::new(TlsAcceptor::from(sign_default_certificates().unwrap())));

static TEMP_CONNECTION_POOL: LazyLock<Arc<BackendConnectionPool>> = LazyLock::new(|| {
    BackendConnectionPool::new(BackendConnectionPoolConfig::new(SocketAddr::V4(
        SocketAddrV4::new("192.168.2.254".parse().unwrap(), 5244),
    )))
});

async fn accept(listener: CustomDualStackTcpListener) {
    event!(
        Level::INFO,
        "Listening on {:?}",
        listener.local_addrs().unwrap()
    );
    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(v) => v,
            Err(e) => {
                println!("Error accepting connection: {}", e);
                continue;
            }
        };
        tokio::spawn(async move {
            let r = handle_connection(stream, addr).await;
            if let Err(e) = r {
                println!("Error handling connection: {}", e);
            }
        });
    }
}

pub async fn listen(port: u16) -> anyhow::Result<()> {
    let thread = tokio::spawn(async move {
        let listener = CustomDualStackTcpListener::new_by_port(port).await.unwrap();
        accept(listener).await;
    });

    LISTENERS.write().await.insert(port, thread);
    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    event!(Level::INFO, "Connection from {}", addr);
    let stream = BufferStream::new(WrapperBufferStream::Raw(stream));
    let (stream, proxy_protocol) = protocols::get_proxy_protocol(stream).await?;
    // event!(Level::INFO, "Proxy protocol: {:?}", proxy_protocol);
    let (stream, tls) = protocols::get_tls_sni(stream).await?;
    let final_stream = match tls {
        Some(tls) => {
            let s = DEFAULT_TLS_CONFIG.accept(stream).await?;
            BufferStream::new(WrapperBufferStream::TlsServerBufferStream(Box::new(s)))
        }
        None => stream,
    };
    // if is proxyprotocol

    let io = TokioIo::new(final_stream);
    let r = HTTP_BUILDER.serve_connection(io, service_fn(handle)).await;
    if let Err(e) = r {
        println!("Error serving connection: {}", e);
    }
    Ok(())
}

pub async fn handle(req: Request<Incoming>) -> anyhow::Result<hyper::Response<Incoming>> {
    // let conn = TcpStream::connect(("192.168.2.254", 23333)).await?;
    let conn = TEMP_CONNECTION_POOL.get().await?;
    let io = TokioIo::new(conn);
    let (mut c_req, connection) = client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });
    Ok(c_req.send_request(req).await?)
}
