use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use tokio::net::{TcpSocket, TcpStream};
use tokio_dual_stack::{DualStackTcpListener, Tcp};
use utils::backlog::LISTEN_BACKLOG_SIZE;

#[derive(Debug)]
pub struct CustomDualStackTcpListener {
    inner: DualStackTcpListener,
}

impl CustomDualStackTcpListener {
    pub fn new(v6_addr: SocketAddrV6, v4_addr: SocketAddrV4) -> Result<Self, std::io::Error> {
        let v4 = TcpSocket::new_v4().unwrap();
        let v6 = TcpSocket::new_v6().unwrap();
        v6.set_reuseaddr(true)?;
        v4.set_reuseaddr(true)?;
        v4.set_reuseport(true)?;
        v6.set_reuseport(true)?;
        // bind
        v4.bind(v4_addr.into()).unwrap();
        v6.bind(v6_addr.into()).unwrap();
        Ok(Self {
            inner: DualStackTcpListener::from_sockets(
                (v6, LISTEN_BACKLOG_SIZE as u32),
                (v4, LISTEN_BACKLOG_SIZE as u32),
            )?,
        })
    }

    pub async fn new_by_port(port: u16) -> Result<Self, std::io::Error> {
        Self::new(
            SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0),
            SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port),
        )
    }

    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr), std::io::Error> {
        self.inner.accept().await
    }

    pub fn local_addrs(&self) -> Result<(SocketAddrV6, SocketAddrV4), std::io::Error> {
        self.inner.local_addr()
    }
}
