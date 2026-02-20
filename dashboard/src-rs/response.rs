use std::time::Instant;

use anyhow::Result;
use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode, header::CONTENT_TYPE},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use serde::Serialize;
// use serde_json::{json, Value};
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{Level, event};

#[derive(Debug, Clone, Serialize)]
struct InnerAPIResponse<T: Serialize = ()> {
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

// ---------- 统一的 API 响应结构 ----------
#[derive(Debug, Clone)]
pub struct APIResponse<T: Serialize = ()> {
    inner: InnerAPIResponse<T>,
    pub headers: HeaderMap,
}

impl<T: Serialize> APIResponse<T> {
    pub fn status(&self) -> u16 {
        self.inner.status
    }

    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }

    pub fn data(&self) -> Option<&T> {
        self.inner.data.as_ref()
    }

    pub fn new(data: Option<T>, status: u16, message: Option<&str>) -> Self {
        Self {
            inner: InnerAPIResponse {
                status,
                data,
                message: message.map(|s| s.to_string()),
            },
            headers: HeaderMap::new(),
        }
    }

    pub fn ok(data: T) -> Self {
        Self::new(Some(data), 200, None)
    }

    pub fn error(data: Option<T>, status: u16, message: impl Into<String>) -> Self {
        Self::new(data, status, Some(message.into().as_str()))
    }

    pub fn result(data: Result<T>) -> Self {
        match data {
            Ok(data) => Self::ok(data),
            Err(e) => Self::error(None, 500, e.to_string()),
        }
    }
}

impl<T: Serialize> IntoResponse for APIResponse<T> {
    fn into_response(self) -> Response<Body> {
        let status =
            StatusCode::from_u16(self.inner.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = serde_json::to_string(&self.inner).unwrap();
        let mut headers = self.headers.clone();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let mut builder = Response::builder().status(status);

        let response_headers = builder.headers_mut();
        if let Some(response_headers) = response_headers {
            *response_headers = headers;
        }
        builder.body(Body::new(body)).unwrap()
    }
}

// ---------- 全局错误类型（自动转为 APIResponse）----------
#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound(String),
    Internal(anyhow::Error), // 包裹任意底层错误
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", e),
            ),
        };
        APIResponse::<()>::error(None, status.as_u16(), message).into_response()
    }
}

// 将 anyhow::Error 转换为 AppError::Internal
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::Internal(err.into())
    }
}

// AppError into APIResponse
impl From<AppError> for APIResponse {
    fn from(err: AppError) -> Self {
        let (status, message) = match err {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", e),
            ),
        };
        Self::error(None, status.as_u16(), message)
    }
}

// ---------- 请求日志中间件 ----------
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct ReqRecord {
    method: String,
    raw_path: String,
    user_agent: Option<String>,
    host: Option<String>,
    remote_addr: String, // 可以从扩展中获取真实 IP
}

pub async fn logging_middleware(req: Request<Body>, next: Next) -> Response<Body> {
    let record = ReqRecord {
        method: req.method().to_string(),
        raw_path: req.uri().path().to_string(),
        user_agent: req
            .headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        host: req
            .headers()
            .get("host")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        remote_addr: "".to_string(), // 实际可从 `req.extensions()` 获取
    };

    let start = Instant::now();
    let response = next.run(req).await;
    let elapsed = start.elapsed();

    event!(
        Level::INFO,
        "{} {} {} {}ms",
        record.method,
        record.raw_path,
        response.status().as_u16(),
        elapsed.as_micros() as f64 / 1000.0 // 毫秒
    );

    response
}

pub fn wrapper_router(router: Router) -> Router {
    router
        .layer(middleware::from_fn(logging_middleware))
        .layer(CatchPanicLayer::new())
}
