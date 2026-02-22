use std::{net::SocketAddr, time::SystemTime};

use axum::{
    Router,
    extract::{ConnectInfo, Json, Query},
    routing::{get, post},
};
use shared::database::get_database;

use crate::{
    database::auth::Authentication,
    foundation::RemoteAddr,
    models::auth::{AuthJWTInfoExtract, AuthPostBody, AuthQueryInfo, AuthResponse, AuthResponseInfo},
    response::APIResponse,
};

// use totp give a token
mod foundation;
mod jwt;
mod totp;

pub use foundation::{DEFAULT_ADMIN_USERNAME, generate_random_secret};
pub use jwt::{get_user_info_from_verify_jwt, sign_jwt};
pub use totp::get_totp_code;

#[axum::debug_handler]
pub async fn login(
    ConnectInfo(RemoteAddr(addr)): ConnectInfo<RemoteAddr>,
    Json(body): Json<AuthPostBody>,
) -> APIResponse<AuthResponse> {
    let now = SystemTime::now();
    let res = inner_login(body, addr).await;
    if res.status() == 200 {
        return res;
    }
    // sleep random, maybe 2 ~ 5 sec
    let duration = now.elapsed().unwrap().as_millis();
    let sleep_time = std::cmp::max(0, rand::random_range(2000..5000) - duration);
    tokio::time::sleep(std::time::Duration::from_millis(sleep_time as u64)).await;
    res
}

async fn inner_login(body: AuthPostBody, addr: SocketAddr) -> APIResponse<AuthResponse> {
    if body.username.trim().is_empty() || body.totp.trim().is_empty() {
        return APIResponse::error(None, 401, "Invalid username or totp code");
    }
    match get_database()
        .verify_totp(&body.username, &body.totp, addr)
        .await
    {
        Ok(true) => match sign_jwt(&body.username).await {
            Ok(res) => APIResponse::ok(res),
            Err(e) => APIResponse::error(None, 500, e.to_string()),
        },
        Ok(false) => APIResponse::error(None, 401, "Invalid TOTP code"),
        Err(e) => APIResponse::error(None, 500, e.to_string()),
    }
}

pub async fn info(AuthJWTInfoExtract(info): AuthJWTInfoExtract) -> APIResponse<AuthResponseInfo> {
    APIResponse::ok(AuthResponseInfo {
        id: info.user.id,
        username: info.user.username,
        created_at: info.user.created_at,
        updated_at: info.user.updated_at,
    })
}

pub async fn get_userinfo(
    AuthJWTInfoExtract(_): AuthJWTInfoExtract,
    Query(info): Query<AuthQueryInfo>,
) -> APIResponse<AuthResponseInfo> {
    let user = get_database().get_user_from_id(&info.user_id).await;
    match user {
        Ok(user) => APIResponse::ok(AuthResponseInfo {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }),
        Err(_) => APIResponse::error(None, 404, "User not found"),
    }

}
pub async fn refresh_token(
    AuthJWTInfoExtract(info): AuthJWTInfoExtract,
) -> APIResponse<AuthResponse> {
    match sign_jwt(&info.user.username).await {
        Ok(res) => APIResponse::ok(res),
        Err(e) => APIResponse::error(None, 500, e.to_string()),
    }
}

pub fn get_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/info", get(get_userinfo))
        .route("/", get(info))
}
