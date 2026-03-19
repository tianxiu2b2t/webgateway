use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Error;
use bytes::Bytes;
use http_body::Frame;
use http_body_util::Full;
use hyper::{
    Response,
    body::{Body, Incoming},
};
use shared::objectid::ObjectId;

use crate::access::{update_request_size_log, update_response_size_log};

// 创建一个统一的 body 类型
#[derive(Debug)]
pub enum CResponse {
    Incoming(StatisticsIncoming),
    Error(Full<Bytes>),
}

#[derive(Debug)]
pub enum StatisticsIncomingType {
    Request,
    Response,
}

#[derive(Debug)]
pub struct StatisticsIncoming {
    inner: Incoming,
    id: ObjectId,
    method: StatisticsIncomingType,
    size: usize,
}

impl StatisticsIncoming {
    pub fn new(id: ObjectId, inner: Incoming, method: StatisticsIncomingType) -> Self {
        Self {
            inner,
            id,
            method,
            size: 0,
        }
    }

    pub fn real_size_hint(&self) -> usize {
        self.size
    }
}

impl Drop for StatisticsIncoming {
    fn drop(&mut self) {
        match self.method {
            StatisticsIncomingType::Request => {
                update_request_size_log(self.id, self.size);
            }
            StatisticsIncomingType::Response => update_response_size_log(self.id, self.size),
        }
    }
}

impl http_body::Body for StatisticsIncoming {
    type Data = bytes::Bytes;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    /// Attempt to pull out the next data buffer of this stream.
    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, <CResponse as Body>::Error>>> {
        let res = Pin::new(&mut self.inner).poll_frame(cx).map(|opt| {
            opt.map(|result| {
                result.map_err(|e| anyhow::anyhow!(e).into()).map(|v| {
                    v.map_data(|data| {
                        self.size += data.len();
                        data
                    })
                })
            })
        });

        // is end stream
        if self.inner.is_end_stream() {
            match self.method {
                StatisticsIncomingType::Request => {
                    update_request_size_log(self.id, self.size);
                }
                StatisticsIncomingType::Response => update_response_size_log(self.id, self.size),
            }
        }

        res
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> http_body::SizeHint {
        self.inner.size_hint()
    }
}

impl From<String> for CResponse {
    fn from(s: String) -> Self {
        CResponse::Error(Full::new(Bytes::from(s)))
    }
}

impl CResponse {
    pub fn new_from_string(value: impl Into<String>) -> Self {
        CResponse::from(value.into())
    }
}

impl http_body::Body for CResponse {
    type Data = bytes::Bytes;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    /// Attempt to pull out the next data buffer of this stream.
    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, <CResponse as Body>::Error>>> {
        match &mut *self {
            Self::Incoming(incoming) => Pin::new(incoming)
                .poll_frame(cx)
                .map(|opt| opt.map(|result| result.map_err(|e| anyhow::anyhow!(e).into()))),
            Self::Error(full) => Pin::new(full)
                .poll_frame(cx)
                .map(|opt| opt.map(|result| result.map_err(|_| anyhow::anyhow!("error").into()))),
        }
    }

    fn is_end_stream(&self) -> bool {
        match self {
            Self::Incoming(incoming) => incoming.is_end_stream(),
            Self::Error(full) => full.is_end_stream(),
        }
    }

    fn size_hint(&self) -> http_body::SizeHint {
        match self {
            Self::Incoming(incoming) => incoming.size_hint(),
            Self::Error(full) => full.size_hint(),
        }
    }
}

pub enum CResponseResult {
    Backend(Response<CResponse>),
    NotFoundGateway,
    GatewayError(Error),
    BadRequest,
    Timeout,
}

pub enum CFirstResponse {
    Error,
    Response(CResponseResult),
}
