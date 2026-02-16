use protocols::{
    proxyprotocol::PRROXY_PROTOCOL_READ_BUF_SIZE,
    tls::{ProtocolTLS, TLS_HANDSHAKE_START_LENGTH, get_tls_sni_from_buf, is_tls_handshake},
};
use tracing::event;

use shared::streams::BufferStream;

pub trait SimpleReadExt {
    fn pre_read_buf(&mut self, size: usize) -> impl Future<Output = tokio::io::Result<Vec<u8>>>;
}

impl SimpleReadExt for BufferStream {
    async fn pre_read_buf(&mut self, size: usize) -> tokio::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        let size = self.pre_read(&mut buf).await?;
        Ok(buf[..size].to_owned())
    }
}

#[allow(unused)]
pub async fn get_proxy_protocol(
    mut stream: BufferStream,
) -> anyhow::Result<(BufferStream, Option<()>)> {
    let buf = stream.pre_read_buf(PRROXY_PROTOCOL_READ_BUF_SIZE).await?;
    println!("buf: {:?}", &buf);
    // TODO: implement
    Ok((stream.into_inner(), None))
}

pub async fn get_tls_sni(
    mut stream: BufferStream,
) -> anyhow::Result<(BufferStream, Option<ProtocolTLS>)> {
    let mut data = stream.pre_read_buf(TLS_HANDSHAKE_START_LENGTH).await?;
    println!("data: {:?}", &data);
    if !is_tls_handshake(&data) {
        return Ok((stream.into_inner(), None));
    }
    data.extend(stream.pre_read_buf(8192).await?);
    loop {
        match get_tls_sni_from_buf(&data) {
            Ok(sni) => {
                event!(tracing::Level::INFO, "TLS SNI: {:?}", &sni);
                return Ok((stream, sni));
            }
            Err(protocols::tls::ProtocolTLSError::WantMoreData(Some(n))) => {
                data.extend(stream.pre_read_buf(n).await?);
            }
            Err(e) => {
                event!(tracing::Level::ERROR, "TLS SNI error: {:?}", e);
            }
        }
    }
    // Ok((stream.into_inner(), None))
}
