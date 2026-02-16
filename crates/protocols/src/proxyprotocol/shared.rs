use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Clone, Copy)]
pub enum ProxyProtocolError {
    InvaildIp,
    InvaildPort,
    WantMoreData,
}

pub type ProxyProtocolResult<T> = std::result::Result<T, ProxyProtocolError>;

pub fn parse_sock_addr(ip_buf: &[u8], port_buf: &[u8]) -> ProxyProtocolResult<SocketAddr> {
    if port_buf.len() != 2 {
        return Err(ProxyProtocolError::InvaildPort);
    }
    Ok(SocketAddr::new(
        parse_ip_addr(ip_buf)?,
        u16::from_be_bytes(port_buf.try_into().unwrap()),
    ))
}

pub fn parse_ip_addr(buf: &[u8]) -> ProxyProtocolResult<IpAddr> {
    if buf.len() != 4 && buf.len() != 16 {
        return Err(ProxyProtocolError::InvaildIp);
    }
    // 4 bytes
    if buf.len() == 4 {
        let ip = u32::from_be_bytes(buf.try_into().unwrap());
        return Ok(IpAddr::V4(ip.into()));
    }
    let ipv6_buf: [u8; 16] = buf.try_into().unwrap();
    // 16 bytes
    Ok(IpAddr::from(ipv6_buf))
}
