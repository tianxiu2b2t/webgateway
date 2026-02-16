use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

mod shared;
mod v1;
mod v2;

#[derive(Debug, Clone, Copy)]
pub enum ProxyProtocolVersion {
    /// Version 1
    V1,
    /// Version 2
    V2,
}

#[derive(Debug, Clone)]
pub enum ProxyProtocol {
    V1(ProxyProtocolV1),
    V1Heartbeat,
    V2(ProxyProtocolV2),
}

#[derive(Debug, Clone, Copy)]
pub enum SocketType {
    TCP,
    UDP,
    Unix,
}

#[derive(Debug, Clone, Copy)]
pub enum AddrType {
    V4,
    V6,
}

#[derive(Debug, Clone)]
pub struct ProxyProtocolV1 {
    socket_type: SocketType,
    src_addr: SocketAddr,
    dst_addr: SocketAddr,
}

impl ProxyProtocolV1 {
    pub fn socket_type(&self) -> SocketType {
        self.socket_type
    }

    pub fn src_addr(&self) -> SocketAddr {
        self.src_addr
    }

    pub fn dst_addr(&self) -> SocketAddr {
        self.dst_addr
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProxyProtocolV2Type {
    Unknown = 0x00,
    IPv4 = 0x10,
    IPv6 = 0x20,
    UnixStream = 0x30,
}

impl From<u8> for ProxyProtocolV2Type {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ProxyProtocolV2Type::Unknown,
            0x10 => ProxyProtocolV2Type::IPv4,
            0x20 => ProxyProtocolV2Type::IPv6,
            0x30 => ProxyProtocolV2Type::UnixStream,
            _ => ProxyProtocolV2Type::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProxyProtocolV2 {
    socket_type: SocketType,
    protocol: ProxyProtocolV2Type,
    src_addr: Option<SocketAddr>,
    dst_addr: Option<SocketAddr>,
}

impl ProxyProtocolV2 {
    pub fn socket_type(&self) -> SocketType {
        self.socket_type
    }

    pub fn protocol(&self) -> ProxyProtocolV2Type {
        self.protocol
    }

    pub fn src_addr(&self) -> Option<SocketAddr> {
        self.src_addr
    }

    pub fn dst_addr(&self) -> Option<SocketAddr> {
        self.dst_addr
    }
}

#[derive(Debug, Clone)]
pub enum ProxyProtocolError {
    InvalidVersion,
    InvalidFormat,
    InvaildIp,
    InvaildPort,
}

pub type ProxyProtocolResult<T> = Result<T, ProxyProtocolError>;

const PROXY_PROTOCOL_V1_BEGIN_DATA: &[u8; 6] = b"PROXY ";
const PROXY_PROTOCOL_V1_UNKNOWN: &[u8; 7] = b"UNKNOWN";
const PROXY_PROTOCOL_V1_END_DATA: &[u8; 2] = b"\r\n";
const PROXY_PROTOCOL_V2_BEGIN_DATA: &[u8; 12] = b"\r\n\r\n\x00\r\nQUIT\n";
pub const PRROXY_PROTOCOL_READ_BUF_SIZE: usize = 12;

pub fn parse_proxy_protocol_version(buf: &[u8]) -> Option<ProxyProtocolVersion> {
    if buf.starts_with(PROXY_PROTOCOL_V1_BEGIN_DATA) && buf.ends_with(PROXY_PROTOCOL_V1_END_DATA) {
        return Some(ProxyProtocolVersion::V1);
    }
    if buf.starts_with(PROXY_PROTOCOL_V2_BEGIN_DATA) {
        return Some(ProxyProtocolVersion::V2);
    }
    None
}

fn parse_proxy_protocol_v1(buf: &[u8]) -> ProxyProtocolResult<Option<ProxyProtocolV1>> {
    if buf.eq(PROXY_PROTOCOL_V1_UNKNOWN) {
        return Ok(None);
    }
    let data: Vec<&[u8]> = buf.split(|&c| c == b' ').collect();
    if data.len() != 5 {
        return Err(ProxyProtocolError::InvalidFormat);
    }

    let (socket_type, addr_type) = match data[0] {
        b"TCP4" => (SocketType::TCP, AddrType::V4),
        b"TCP6" => (SocketType::TCP, AddrType::V6),
        b"UDP4" => (SocketType::UDP, AddrType::V4),
        b"UDP6" => (SocketType::UDP, AddrType::V6),
        _ => return Err(ProxyProtocolError::InvalidFormat),
    };

    Ok(Some(ProxyProtocolV1 {
        socket_type,
        src_addr: parse_ip_port(data[1], data[3], &addr_type)?,
        dst_addr: parse_ip_port(data[2], data[4], &addr_type)?,
    }))

    // Ok(None)
}

fn parse_proxy_protocol_v2(buf: &[u8]) -> ProxyProtocolResult<ProxyProtocolV2> {
    let (version, command) = (buf[0] >> 4, buf[0] & 0x0f);
    let (af, proto) = (buf[1] >> 4, buf[1] & 0x0f);
    let addr_length = u16::from_be_bytes(buf[2..4].try_into().unwrap()) as usize;
    let addr_type = match af {
        0x01 => AddrType::V4,
        0x04 => AddrType::V6,
        _ => return Err(ProxyProtocolError::InvaildIp),
    };
    let src_addr = &buf[4..4 + addr_length];
    let dst_addr = &buf[4 + addr_length..4 + addr_length * 2];
    let src_port = &buf[4 + addr_length * 2..4 + addr_length * 2 + 2];
    let dst_port = &buf[4 + addr_length * 2 + 2..4 + addr_length * 2 + 4];
    let src_sock_addr = parse_ip_port(src_addr, src_port, &addr_type)?;
    let dst_sock_addr = parse_ip_port(dst_addr, dst_port, &addr_type)?;
    Ok(ProxyProtocolV2 {
        socket_type: match command {
            0x00 => SocketType::TCP,
            0x01 => SocketType::UDP,
            _ => return Err(ProxyProtocolError::InvalidFormat),
        },
        protocol: ProxyProtocolV2Type::from(proto),
        src_addr: Some(src_sock_addr),
        dst_addr: Some(dst_sock_addr),
    })
}

pub fn parse_proxy_protocol(buf: &[u8]) -> ProxyProtocolResult<Option<ProxyProtocol>> {
    let version = parse_proxy_protocol_version(buf);
    Ok(match version {
        Some(ProxyProtocolVersion::V1) => Some({
            let result = parse_proxy_protocol_v1(
                &buf[PROXY_PROTOCOL_V1_BEGIN_DATA.len()..PROXY_PROTOCOL_V1_END_DATA.len()],
            )?;
            match result {
                None => ProxyProtocol::V1Heartbeat,
                Some(r) => ProxyProtocol::V1(r),
            }
        }),
        Some(ProxyProtocolVersion::V2) => Some(ProxyProtocol::V2(parse_proxy_protocol_v2(
            &buf[PROXY_PROTOCOL_V2_BEGIN_DATA.len()..],
        )?)),
        None => None,
    })
}

fn parse_ip_port(ip: &[u8], port: &[u8], addr_type: &AddrType) -> ProxyProtocolResult<SocketAddr> {
    let addr = String::from_utf8(ip.to_vec()).map_err(|_| ProxyProtocolError::InvaildIp)?;
    let str_port = String::from_utf8(port.to_vec()).map_err(|_| ProxyProtocolError::InvaildIp)?;
    let port = str_port
        .parse::<u16>()
        .map_err(|_| ProxyProtocolError::InvaildPort)?;
    Ok(SocketAddr::new(
        match addr_type {
            AddrType::V4 => IpAddr::V4(
                addr.parse::<Ipv4Addr>()
                    .map_err(|_| ProxyProtocolError::InvaildIp)?,
            ),
            AddrType::V6 => IpAddr::V6(
                addr.parse::<Ipv6Addr>()
                    .map_err(|_| ProxyProtocolError::InvaildIp)?,
            ),
        },
        port,
    ))
}
