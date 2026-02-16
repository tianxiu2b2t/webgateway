use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufStream, ReadBuf},
    net::TcpStream,
};

#[derive(Debug)]
pub enum WrapperBufferStream {
    Raw(TcpStream),
    TlsClient(Box<tokio_rustls::client::TlsStream<TcpStream>>),
    TlsServer(Box<tokio_rustls::server::TlsStream<TcpStream>>),
    TlsServerBufferStream(Box<tokio_rustls::server::TlsStream<BufferStream>>),
}

impl WrapperBufferStream {
    pub async fn close(self) -> Result<(), std::io::Error> {
        match self {
            WrapperBufferStream::Raw(mut stream) => stream.shutdown().await,
            WrapperBufferStream::TlsClient(mut stream) => stream.shutdown().await,
            WrapperBufferStream::TlsServer(mut stream) => stream.shutdown().await,
            WrapperBufferStream::TlsServerBufferStream(mut stream) => stream.shutdown().await,
        }
    }
}

impl AsyncRead for WrapperBufferStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match &mut *self {
            WrapperBufferStream::Raw(stream) => Pin::new(stream).poll_read(cx, buf),
            WrapperBufferStream::TlsClient(stream) => Pin::new(stream.as_mut()).poll_read(cx, buf),
            WrapperBufferStream::TlsServer(stream) => Pin::new(stream.as_mut()).poll_read(cx, buf),
            WrapperBufferStream::TlsServerBufferStream(stream) => {
                Pin::new(stream.as_mut()).poll_read(cx, buf)
            }
        }
    }
}

impl AsyncWrite for WrapperBufferStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match &mut *self {
            WrapperBufferStream::Raw(stream) => Pin::new(stream).poll_write(cx, buf),
            WrapperBufferStream::TlsClient(stream) => Pin::new(stream.as_mut()).poll_write(cx, buf),
            WrapperBufferStream::TlsServer(stream) => Pin::new(stream.as_mut()).poll_write(cx, buf),
            WrapperBufferStream::TlsServerBufferStream(stream) => {
                Pin::new(stream.as_mut()).poll_write(cx, buf)
            }
        }
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        match &mut *self {
            WrapperBufferStream::Raw(stream) => Pin::new(stream).poll_write_vectored(cx, bufs),
            WrapperBufferStream::TlsClient(stream) => {
                Pin::new(stream.as_mut()).poll_write_vectored(cx, bufs)
            }
            WrapperBufferStream::TlsServer(stream) => {
                Pin::new(stream.as_mut()).poll_write_vectored(cx, bufs)
            }
            WrapperBufferStream::TlsServerBufferStream(stream) => {
                Pin::new(stream.as_mut()).poll_write_vectored(cx, bufs)
            }
        }
    }

    fn is_write_vectored(&self) -> bool {
        match self {
            WrapperBufferStream::Raw(stream) => stream.is_write_vectored(),
            WrapperBufferStream::TlsClient(stream) => stream.is_write_vectored(),
            WrapperBufferStream::TlsServer(stream) => stream.is_write_vectored(),
            WrapperBufferStream::TlsServerBufferStream(stream) => stream.is_write_vectored(),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match &mut *self {
            WrapperBufferStream::Raw(stream) => Pin::new(stream).poll_flush(cx),
            WrapperBufferStream::TlsClient(stream) => Pin::new(stream.as_mut()).poll_flush(cx),
            WrapperBufferStream::TlsServer(stream) => Pin::new(stream.as_mut()).poll_flush(cx),
            WrapperBufferStream::TlsServerBufferStream(stream) => {
                Pin::new(stream.as_mut()).poll_flush(cx)
            }
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match &mut *self {
            WrapperBufferStream::Raw(stream) => Pin::new(stream).poll_shutdown(cx),
            WrapperBufferStream::TlsClient(stream) => Pin::new(stream.as_mut()).poll_shutdown(cx),
            WrapperBufferStream::TlsServer(stream) => Pin::new(stream.as_mut()).poll_shutdown(cx),
            WrapperBufferStream::TlsServerBufferStream(stream) => {
                Pin::new(stream.as_mut()).poll_shutdown(cx)
            }
        }
    }
}

#[derive(Debug)]
pub struct BufferStream {
    inner: BufStream<WrapperBufferStream>,
    pre_buffer: usize,
}

impl BufferStream {
    pub fn new(stream: WrapperBufferStream) -> Self {
        Self {
            inner: BufStream::new(stream),
            pre_buffer: 0,
        }
    }

    pub fn new_with_capacity(stream: WrapperBufferStream, capacity: usize) -> Self {
        Self {
            inner: BufStream::with_capacity(capacity, 0, stream),
            pre_buffer: 0,
        }
    }

    pub async fn pre_read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let filled = self.inner.fill_buf().await?;
        let filled_len = filled.len();
        let available = filled_len - self.pre_buffer; // bytes not yet "taken" from this buffer
        if available == 0 {
            return Ok(0);
        }
        let n = std::cmp::min(buf.len(), available);
        buf[..n].copy_from_slice(&filled[self.pre_buffer..self.pre_buffer + n]);
        self.pre_buffer += n;
        // If you want to permanently consume these bytes from the BufStream:
        // self.inner.consume(n);
        // self.pre_buffer = 0; // reset offset because we consumed
        Ok(n)
    }

    // pub fn consume(&mut self, amt: usize) {
    //     self.pre_buffer -= amt;
    //     self.inner.consume(amt);
    // }

    pub fn into_inner(self) -> BufferStream {
        Self {
            inner: self.inner,
            pre_buffer: 0,
        }
    }

    // pub fn consume_all_and_into_inner(mut self) -> Self {
    //     self.inner.consume(self.pre_buffer);
    //     self.into_inner()
    // }
}

impl AsyncRead for BufferStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for BufferStream {
    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }

    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write_vectored(cx, bufs)
    }
}
