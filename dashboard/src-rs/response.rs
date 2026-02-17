use std::time::Instant;

use anyhow::Result;
use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{StatusCode, header::CONTENT_TYPE},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use serde::Serialize;
// use serde_json::{json, Value};
use tower_http::catch_panic::CatchPanicLayer;
use tracing::{Level, event};

// ---------- 统一的 API 响应结构 ----------
#[derive(Debug, Clone, Serialize)]
pub struct APIResponse<T: Serialize = ()> {
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> APIResponse<T> {
    pub fn new(data: Option<T>, status: u16, message: Option<&str>) -> Self {
        Self {
            data,
            status,
            message: message.map(|m| m.to_string()),
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
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap()
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
            AppError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal server error: {}", e)),
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

// ---------- 全局 404 处理器 ----------
// async fn fallback_handler(uri: axum::http::Uri) -> impl IntoResponse {
//     AppError::NotFound(format!("No route for {}", uri))
// }

// // ---------- 业务 Handler 示例 ----------
// async fn hello() -> Result<APIResponse<Value>, AppError> {
//     Ok(APIResponse::ok(json!({"message": "Hello World"})))
// }

// async fn fail() -> Result<APIResponse<Value>, AppError> {
//     Err(AppError::BadRequest("参数错误".to_string()))
// }

// async fn panic_handler() -> Result<APIResponse<Value>, AppError> {
//     panic!("模拟 panic");
// }

// async fn internal_error() -> Result<APIResponse<Value>, AppError> {
//     // 模拟任意库返回的 anyhow::Error
//     let _: () = Err(anyhow::anyhow!("数据库连接失败"))?;
//     Ok(APIResponse::ok(json!(null)))
// }

pub fn wrapper_router(router: Router) -> Router {
    router
        .layer(middleware::from_fn(logging_middleware))
        .layer(CatchPanicLayer::new())
}
