use std::time::SystemTime;

use axum::{Router, extract::Json, routing::{get, post}};
use shared::database::get_database;

use crate::{
    database::auth::Authentication,
    models::auth::{AuthJWTInfoExtract, AuthPostBody, AuthResponse, AuthResponseInfo},
    response::APIResponse,
};

// use totp give a token
mod foundation;
mod jwt;
mod totp;

pub use foundation::{DEFAULT_ADMIN_USERNAME, generate_random_secret};
pub use jwt::{sign_jwt, get_user_info_from_verify_jwt};
pub use totp::get_totp_code;

pub async fn login(Json(body): Json<AuthPostBody>) -> APIResponse<AuthResponse> {
    let now = SystemTime::now();
    let res = inner_login(body).await;
    if res.status == 200 {
        return res;
    }
    // sleep random, maybe 2 ~ 5 sec
    let duration = now.elapsed().unwrap().as_millis();
    let sleep_time = std::cmp::max(0, rand::random_range(2000..5000) - duration);
    tokio::time::sleep(std::time::Duration::from_millis(sleep_time as u64)).await;
    res
}

async fn inner_login(body: AuthPostBody) -> APIResponse<AuthResponse> {
    if body.username.trim().is_empty() || body.totp.trim().is_empty() {
        return APIResponse::error(None, 401, "Invalid username or totp code");
    }
    match get_database().verify_totp(&body.username, &body.totp).await {
        Ok(true) => match sign_jwt(&body.username).await {
            Ok(res) => APIResponse::ok(res),
            Err(e) => APIResponse::error(None, 500, e.to_string()),
        },
        Ok(false) => APIResponse::error(None, 401, "Invalid TOTP code"),
        Err(e) => APIResponse::error(None, 500, e.to_string()),
    }
}

pub async fn info(
    AuthJWTInfoExtract(info): AuthJWTInfoExtract,
) -> APIResponse<AuthResponseInfo> {
    APIResponse::ok(AuthResponseInfo {
        username: info.user.username,
    })
}

pub fn get_router() -> Router {
    Router::new().route("/login", post(login)).route("/", get(info))
}
