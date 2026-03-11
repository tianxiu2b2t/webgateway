use std::{
    net::SocketAddr, sync::{Arc, LazyLock}, time::Duration
};

use dashmap::DashMap;
use hyper::{
    Request, Response, Version, body::Incoming, client, service::service_fn
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
    response::CResponse, state::{BaseClientState, ClientState}, sync::{SERVER_CONFIG, websites::get_website}
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
            Err(_) => {
                continue;
            }
        };
        tokio::spawn(async move {
            let _ = handle_connection(stream, addr).await;
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
    let local_addr = stream.local_addr()?;
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
    let state = Arc::new(BaseClientState {
        tls,
        remote_addr: addr.ip(),
        local_addr: local_addr.ip(),
    });
    let io = TokioIo::new(final_stream);
    let _ = HTTP_BUILDER
        .serve_connection(
            io,
            service_fn(move |req: Request<Incoming>| {
                let state = state.clone();
                handle(req, state)
            }),
        )
        .await;
    Ok(())
}


pub async fn handle(
    req: Request<Incoming>,
    base_state: Arc<BaseClientState>,
) -> anyhow::Result<hyper::Response<CResponse>> {
    let host = base_state.tls.clone().map(|v| v.hostname).unwrap_or_else(|| {
        req.headers()
            .get("host")
            .and_then(|v| v.to_str().ok().map(|v| v.to_string()))
    }).unwrap_or_else(|| {
        req.uri().host().map(|v| v.to_string()).unwrap_or_default()
    });
    let site = match get_website(&host)
        .await {
        Some(v) => v,
        None => {
            let resp = Response::builder().status(404).header("Server", "WebGateway").body(CResponse::new_from_string("Not Found Gateway"))?;
            return Ok(resp);
        }
    };
    let state = ClientState {
        base: base_state,
        website: site.clone(),
        host,
    };
    let resp = timeout(Duration::from_secs(60), inner_handle(req, state)).await;
    match resp {
        Ok(v) =>  {
            match v {
                Ok(v) => Ok(v),
                Err(e) => {
                    eprintln!("Error handling request: {}", e);
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
    origin_req: Request<Incoming>,
    state: ClientState,
) -> anyhow::Result<hyper::Response<CResponse>> {
    let site = &state.website;

    let pool = site.pool();
    let conn = pool.get().await?;
    let io = TokioIo::new(conn);
    let (mut c_req, connection) = client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });
    let origin_version = origin_req.version();
    let mut req = Request::builder().method(origin_req.method()).version(Version::HTTP_11);
    if let Some(v) = req.headers_mut() { v.extend(origin_req.headers().clone()) }
    if let Some(v) = req.extensions_mut() { v.extend(origin_req.extensions().clone()) }
    req = req.uri(pool.get_path().map_or_else(|| origin_req.uri().path().to_string(), |v| {
        let a = v.join(origin_req.uri().path()).unwrap();
        a.path().to_string()
    }));
    
    // insert custom headers
    let headers = req.headers_mut().unwrap();
    headers.insert("Host", state.host.parse()?);
    headers.insert("X-Real-Ip", format!("{}", &state.remote_addr()).parse()?);
    headers.insert("X-Forwarded-For", format!("{}", state.remote_addr()).parse()?);
    headers.insert(
        "X-Forwarded-Proto",
        state.scheme()
        .to_string()
        .parse()?,
    );
    headers.insert("X-Forwarded-Host", state.host.parse()?);
    let final_req = req.body(origin_req.into_body()).unwrap();

    let mut resp = c_req.send_request(final_req).await?;
    resp.headers_mut().insert("Server", "WebGateway".parse()?);
    let (mut parts, b) = resp.into_parts();
    parts.version = origin_version;
    let final_resp = Response::from_parts(parts, CResponse::Incoming(b));
    Ok(final_resp)
}
