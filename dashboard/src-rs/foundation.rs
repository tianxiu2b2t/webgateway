use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{FromRequestParts, connect_info::Connected}, http::request::Parts, serve::{IncomingStream, Listener}
};
use shared::listener::CustomDualStackTcpListener;
use tokio::net::TcpStream;

use crate::response::APIResponse;

#[derive(Debug)]
pub struct CListener(CustomDualStackTcpListener);

impl Listener for CListener {
    type Io = TcpStream;

    type Addr = SocketAddr;

    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        self.0.accept().await.unwrap()
    }

    fn local_addr(&self) -> tokio::io::Result<Self::Addr> {
        Ok(SocketAddr::V6(self.0.local_addrs()?.0))
    }
}

// into Listener
impl From<CustomDualStackTcpListener> for CListener {
    fn from(listener: CustomDualStackTcpListener) -> Self {
        Self(listener)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RemoteAddr(pub IpAddr);

// into SocketAddr
impl From<RemoteAddr> for IpAddr {
    fn from(addr: RemoteAddr) -> Self {
        addr.0
    }
}

impl From<IpAddr> for RemoteAddr {
    fn from(addr: IpAddr) -> Self {
        RemoteAddr(addr)
    }
}

impl<S> FromRequestParts<S> for RemoteAddr where S: Sync + Send {
    type Rejection = APIResponse<()>;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // let authorization = parts
        //     .headers
        //     .get("Authorization")
        //     .ok_or(AppError::BadRequest("Invaild Ip".to_string()))?;
        // // remove "Bearer " from the header
        // let token = authorization.to_str().unwrap().replace("Bearer ", "");
        // let info = get_user_info_from_verify_jwt(&token)
        //     .await
        //     .ok()
        //     .ok_or(AppError::Unauthorized)?;
        // Ok(Self(info))
        let ip = parts.extensions.get::<Self>().unwrap();
        Ok(*ip)
    }
}

impl<'a> Connected<IncomingStream<'a, CListener>> for RemoteAddr {
    fn connect_info(target: IncomingStream<'a, CListener>) -> Self {
        RemoteAddr(target.remote_addr().ip())
    }
}
