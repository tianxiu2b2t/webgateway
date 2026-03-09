use std::{
    net::SocketAddr,
    sync::{Arc, LazyLock}, time::Duration,
};

use anyhow::anyhow;
use dashmap::DashMap;
use hyper::{
    Request, Response,
    body::Incoming,
    client,
    service::service_fn,
};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use shared::{
    listener::CustomDualStackTcpListener,
    streams::{BufferStream, WrapperBufferStream},
};
use tokio::{net::TcpStream, task::JoinHandle, time::timeout};
use tokio_rustls::TlsAcceptor;
use tracing::{Level, event};

use crate::{
    response::CResponse, state::ClientState, sync::{SERVER_CONFIG, websites::get_website}
};
pub mod backends;
pub mod protocols;

static HTTP_BUILDER: LazyLock<Builder<TokioExecutor>> = LazyLock::new(|| {
    hyper_util::server::conn::auto::Builder::<TokioExecutor>::new(TokioExecutor::new())
});

static LISTENERS: LazyLock<DashMap<u16, JoinHandle<()>>> = LazyLock::new(DashMap::default);

static TLS_ACCEPTOR: LazyLock<Arc<TlsAcceptor>> =
    LazyLock::new(|| Arc::new(TlsAcceptor::from(SERVER_CONFIG.clone())));

async fn accept(listener: CustomDualStackTcpListener) {
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
    if LISTENERS.contains_key(&port) {
        return Ok(());
    }
    let thread = tokio::spawn(async move {
        let listener = CustomDualStackTcpListener::new_by_port(port).await.unwrap();
        event!(
            Level::INFO,
            "Listening on {:?}",
            listener.local_addrs().unwrap()
        );
        accept(listener).await;
    });

    LISTENERS.insert(port, thread);
    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    event!(Level::INFO, "Connection from {}", addr);
    let stream = BufferStream::new(WrapperBufferStream::Raw(stream));
    let (stream, _) = protocols::get_proxy_protocol(stream).await?;
    // event!(Level::INFO, "Proxy protocol: {:?}", proxy_protocol);
    let (stream, tls) = protocols::get_tls_sni(stream).await?;
    let final_stream = match &tls {
        Some(_) => {
            let s = TLS_ACCEPTOR.accept(stream).await?;
            BufferStream::new(WrapperBufferStream::TlsServerBufferStream(Box::new(s)))
        }
        None => stream,
    };
    // if is proxyprotocol
    let state = Arc::new(ClientState {
        tls,
        remote_addr: addr.ip(),
    });
    let io = TokioIo::new(final_stream);
    let r = HTTP_BUILDER
        .serve_connection(
            io,
            service_fn(move |req: Request<Incoming>| {
                let state = state.clone();
                handle(req, state)
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
) -> anyhow::Result<hyper::Response<CResponse>> {
    let resp = timeout(Duration::from_secs(60), inner_handle(req, state)).await;
    match resp {
        Ok(v) =>  {
            match v {
                Ok(v) => Ok(v),
                Err(_) => {
                    let resp = Response::builder().status(502).header("Server", "WebGateway").body(CResponse::new_from_string("Bad Gateway"))?;
                    Ok(resp)
                }
            }
        },
        Err(_) => {
            let resp = Response::builder().status(522).header("Server", "WebGateway").body(CResponse::new_from_string("Timeout"))?;
            Ok(resp)
        }
    }
}

async fn inner_handle(
    mut req: Request<Incoming>,
    state: Arc<ClientState>,
) -> anyhow::Result<hyper::Response<CResponse>> {
    let host = state.tls.clone().map(|v| v.hostname).unwrap_or_else(|| {
        req.headers()
            .get("host")
            .and_then(|v| v.to_str().ok().map(|v| v.to_string()))
    });
    if host.is_none() {
        return Err(anyhow!("No Host"));
    }
    let host = host.unwrap();
    let site = get_website(&host)
        .await
        .ok_or(anyhow!("No Runner Website"))?;

    let conn = site.pool().get().await?;
    let io = TokioIo::new(conn);
    let (mut c_req, connection) = client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });
    let headers = req.headers_mut();
    // set X-Real-Ip
    headers.insert("X-Real-Ip", format!("{}", state.remote_addr).parse()?);
    headers.insert("X-Forwarded-For", format!("{}", state.remote_addr).parse()?);
    headers.insert(
        "X-Forwarded-Proto",
        (match state.tls {
            Some(_) => "https",
            None => "http",
        })
        .to_string()
        .parse()?,
    );
    headers.insert("X-Forwarded-Host", host.parse()?);
    let mut resp = c_req.send_request(req).await?;
    resp.headers_mut().insert("Server", "WebGateway".parse()?);
    let (parts, b) = resp.into_parts();
    let final_resp = Response::from_parts(parts, CResponse::Incoming(b));
    Ok(final_resp)
}
