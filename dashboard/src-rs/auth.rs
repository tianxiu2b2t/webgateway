use anyhow::Result;
use axum::{Router, extract::Json, routing::post};
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{
    database::{Database, get_database},
    objectid::ObjectId,
};
use sqlx::{FromRow, Row, postgres::PgRow};
use std::sync::LazyLock;

use crate::{models::auth::AuthPostBody, response::APIResponse};

// use totp give a token

pub static DEFAULT_ADMIN_USERNAME: LazyLock<String> =
    LazyLock::new(|| std::env::var("USERNAME").unwrap_or("admin".to_string()));

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseAuthentication {
    pub id: ObjectId,
    pub username: String,
    pub totp_secret: String,
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
        })
    }
}

pub trait Authentication {
    fn create_user(
        &self,
        username: impl Into<String>,
        totp_secret: &str,
    ) -> impl Future<Output = Result<DatabaseAuthentication>>;
    fn get_user_totp_secret(&self, username: String) -> impl Future<Output = Result<String>>;
    fn init_authentication(&self) -> impl Future<Output = Result<()>>;
    fn is_exists_user(&self, username: &str) -> impl Future<Output = Result<bool>>;
    fn get_user(&self, username: &str) -> impl Future<Output = Result<DatabaseAuthentication>>;
    fn get_first_user(&self) -> impl Future<Output = Result<DatabaseAuthentication>>;
    fn verify_totp(&self, username: &str, totp: &str) -> impl Future<Output = Result<bool>>;
}

impl Authentication for Database {
    async fn init_authentication(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                totp_secret TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                last_login TIMESTAMPTZ,
                last_ip TEXT,
                addresses TEXT[] NOT NULL DEFAULT '{}',
                bound bool NOT NULL DEFAULT false
            );
        "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"CREATE UNIQUE INDEX IF NOT EXISTS users_username_idx ON users (LOWER(username));"#,
        )
        .execute(&self.pool)
        .await?;

        let first_user = match self.get_first_user().await {
            Ok(user) => user,
            Err(e) => {
                println!("Error getting first user: {}", e);
                let secret = generate_totp_secret();
                self.create_user(DEFAULT_ADMIN_USERNAME.clone(), &secret)
                    .await?
            }
        };

        if !first_user.bound {
            let res = get_totp_code(first_user.username, first_user.totp_secret)?;
            println!(
                "The admin user is not binding the totp authenticator, please use the following totp to login: {}",
                res
            );
        }

        Ok(())
    }

    async fn create_user(
        &self,
        username: impl Into<String>,
        totp_secret: &str,
    ) -> Result<DatabaseAuthentication> {
        let username = username.into();
        if self.is_exists_user(&username).await? {
            return Err(anyhow::anyhow!("User already exists"));
        }
        sqlx::query(
            r#"
            INSERT INTO users (id, username, totp_secret)
            VALUES ($1, $2, $3)
        "#,
        )
        .bind(ObjectId::new())
        .bind(&username)
        .bind(totp_secret)
        .execute(&self.pool)
        .await?;

        self.get_user(&username).await
    }

    async fn get_user_totp_secret(&self, username: String) -> Result<String> {
        let totp_secret = sqlx::query_scalar(
            r#"
            SELECT totp_secret
            FROM users
            WHERE LOWER(username) = LOWER($1)
        "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(totp_secret)
    }

    async fn is_exists_user(&self, username: &str) -> Result<bool> {
        let exists = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE LOWER(username) = LOWER($1)
            )
        "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    async fn get_user(&self, username: &str) -> Result<DatabaseAuthentication> {
        let row = sqlx::query(r#"
            SELECT id, username, totp_secret, created_at, updated_at, last_login, last_ip, addresses, bound FROM users
            where LOWER(username) = LOWER($1)
        "#)
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(DatabaseAuthentication::from_row(&row)?)
    }

    async fn get_first_user(&self) -> Result<DatabaseAuthentication> {
        let row = sqlx::query(r#"
            SELECT id, username, totp_secret, created_at, updated_at, last_login, last_ip, addresses, bound FROM users ORDER BY created_at ASC LIMIT 1
        "#)
        .fetch_one(&self.pool)
        .await?;

        Ok(DatabaseAuthentication::from_row(&row)?)
    }

    async fn verify_totp(&self, username: &str, totp: &str) -> Result<bool> {
        let user = self.get_user(username).await?;
        let code = get_totp_code(username, user.totp_secret)?;
        Ok(code == totp)
    }
}

pub fn get_totp_code(username: impl Into<String>, secret: impl Into<Vec<u8>>) -> Result<String> {
    Ok(totp_rs::TOTP::new(
        totp_rs::Algorithm::SHA512,
        6,
        1,
        30,
        secret.into(),
        None,
        username.into(),
    )?
    .generate_current()
    .unwrap())
}

pub fn generate_totp_secret() -> String {
    let mut bytes = [0u8; 32];
    rand::fill(&mut bytes);
    base64::prelude::BASE64_STANDARD.encode(bytes)
}

#[axum::debug_handler]
pub async fn login(Json(body): Json<AuthPostBody>) -> APIResponse<bool> {
    APIResponse::result(get_database().verify_totp(&body.username, &body.totp).await)
}

pub fn get_router() -> Router {
    Router::new().route("/login", post(login))
}
