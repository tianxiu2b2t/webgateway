use std::{
    collections::HashMap,
    net::{SocketAddr, SocketAddrV4},
    sync::{Arc, LazyLock},
};

use ::protocols::tls::ProtocolTLS;
use anyhow::anyhow;
use hyper::{Request, body::Incoming, client, service::service_fn};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use shared::{
    listener::CustomDualStackTcpListener,
    streams::{BufferStream, WrapperBufferStream},
};
use tokio::{net::TcpStream, sync::RwLock, task::JoinHandle};
use tokio_rustls::TlsAcceptor;
use tracing::{Level, event};

use crate::{
    proxy::backends::{BackendConnectionPool, BackendConnectionPoolConfig},
    state::ClientState,
    sync::{SERVER_CONFIG, websites::get_website},
};
pub mod backends;
pub mod protocols;

static HTTP_BUILDER: LazyLock<Builder<TokioExecutor>> = LazyLock::new(|| {
    hyper_util::server::conn::auto::Builder::<TokioExecutor>::new(TokioExecutor::new())
});

static LISTENERS: LazyLock<RwLock<HashMap<u16, JoinHandle<()>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static TLS_ACCEPTOR: LazyLock<Arc<TlsAcceptor>> =
    LazyLock::new(|| Arc::new(TlsAcceptor::from(SERVER_CONFIG.clone())));

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
        event!(
            Level::INFO,
            "Listening on {:?}",
            listener.local_addrs().unwrap()
        );
        accept(listener).await;
    });

    LISTENERS.write().await.insert(port, thread);
    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    event!(Level::INFO, "Connection from {}", addr);
    let stream = BufferStream::new(WrapperBufferStream::Raw(stream));
    let (stream, _) = protocols::get_proxy_protocol(stream).await?;
    // event!(Level::INFO, "Proxy protocol: {:?}", proxy_protocol);
    let (stream, tls) = protocols::get_tls_sni(stream).await?;
    let final_stream = match &tls {
        Some(tls) => {
            let s = TLS_ACCEPTOR.accept(stream).await?;
            BufferStream::new(WrapperBufferStream::TlsServerBufferStream(Box::new(s)))
        }
        None => stream,
    };
    // if is proxyprotocol
    let state = Arc::new(ClientState { tls });
    let io = TokioIo::new(final_stream);
    let r = HTTP_BUILDER
        .serve_connection(
            io,
            service_fn(move |req: Request<Incoming>| {
                let state = state.clone();
                println!("Request: {:?}", req);
                async move { 
                    let resp = handle(req, state).await;
                    println!("Response: {:?}", resp);
                    resp}
            }),
        )
        .await;
    if let Err(e) = r {
        println!("Error serving connection: {}", e);
    }
    Ok(())
}

pub async fn handle(
    req: Request<Incoming>,
    state: Arc<ClientState>,
) -> anyhow::Result<hyper::Response<Incoming>> {
    let host = state.tls.clone().map(|v| v.hostname).unwrap_or_else(|| {
        let h = req
            .headers()
            .get("host")
            .and_then(|v| v.to_str().ok().map(|v| v.to_string()));
        h
    });
    if host.is_none() {
        return Err(anyhow::anyhow!("No host header"));
    }
    let site = get_website(&host.unwrap())
        .await
        .ok_or(anyhow!("No Runner Website"))?;
    // let conn = TcpStream::connect(("192.168.2.254", 23333)).await?;
    let conn = site.pool().get().await?;
    let io = TokioIo::new(conn);
    let (mut c_req, connection) = client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });
    Ok(c_req.send_request(req).await?)
}
