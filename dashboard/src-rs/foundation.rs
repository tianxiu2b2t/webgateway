use std::net::SocketAddr;

use axum::serve::Listener;
use shared::listener::CustomDualStackTcpListener;
use tokio::net::TcpStream;

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
