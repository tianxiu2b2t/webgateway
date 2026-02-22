use axum::{extract::FromRequestParts, http::request::Parts};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::objectid::ObjectId;
use sqlx::{FromRow, Row, postgres::PgRow};

use crate::{
    auth::get_user_info_from_verify_jwt,
    response::{APIResponse, AppError},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthPostBody {
    pub username: String,
    pub totp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub exp_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthJWT {
    pub id: ObjectId,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseAuthentication {
    pub id: ObjectId,
    pub username: String,
    pub totp_secret: String,
    pub jwt_secret: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub last_ip: Option<String>,
    pub addresses: Vec<String>,
    pub bound: bool,
}

impl<'r> FromRow<'r, PgRow> for DatabaseAuthentication {
    fn from_row(row: &'r PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            totp_secret: row.try_get("totp_secret")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            last_login: row.try_get("last_login")?,
            last_ip: row.try_get("last_ip")?,
            addresses: row.try_get("addresses")?,
            bound: row.try_get("bound")?,
            jwt_secret: row.try_get("jwt_secret")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AuthJWTInfo {
    pub user: DatabaseAuthentication,
    pub jwt: AuthJWT,
}

#[derive(Debug, Clone)]
pub struct AuthJWTInfoExtract(pub AuthJWTInfo);

impl<S> FromRequestParts<S> for AuthJWTInfoExtract
where
    S: Send + Sync,
{
    type Rejection = APIResponse<()>;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let authorization = parts
            .headers
            .get("Authorization")
            .ok_or(AppError::Unauthorized)?;
        // remove "Bearer " from the header
        let token = authorization.to_str().unwrap().replace("Bearer ", "");
        let info = get_user_info_from_verify_jwt(&token)
            .await
            .ok()
            .ok_or(AppError::Unauthorized)?;
        Ok(Self(info))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponseInfo {
    pub id: ObjectId,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct AuthQueryInfo {
    pub user_id: ObjectId,
}
