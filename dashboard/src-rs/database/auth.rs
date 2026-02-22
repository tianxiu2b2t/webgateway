// use crate::{
//     auth::{DEFAULT_ADMIN_USERNAME, generate_random_secret, get_totp_code},
//     models::auth::DatabaseAuthentication,
// };
// use anyhow::{Result, anyhow};
// use shared::{database::Database, objectid::ObjectId};
// use sqlx::{FromRow, Row as _};
// use tracing::{Level, event};

// pub trait Authentication {
//     fn create_user(
//         &self,
//         username: impl Into<String>,
//         totp_secret: &str,
//     ) -> impl Future<Output = Result<DatabaseAuthentication>>;
//     // fn get_user_totp_secret(&self, username: String) -> impl Future<Output = Result<String>>;
//     fn init_authentication(&self) -> impl Future<Output = Result<()>>;
//     fn is_exists_user(&self, username: &str) -> impl Future<Output = Result<bool>>;
//     fn get_user(&self, username: &str) -> impl Future<Output = Result<DatabaseAuthentication>>;
//     fn get_first_user(&self) -> impl Future<Output = Result<DatabaseAuthentication>>;
//     fn verify_totp(&self, username: &str, totp: &str) -> impl Future<Output = Result<bool>>;
//     fn get_user_from_id(
//         &self,
//         id: &ObjectId,
//     ) -> impl Future<Output = Result<DatabaseAuthentication>>;
// }

// impl Authentication for Database {
//     async fn init_authentication(&self) -> Result<()> {
//         sqlx::query(
//             r#"
//             CREATE TABLE IF NOT EXISTS users (
//                 id TEXT PRIMARY KEY,
//                 username TEXT NOT NULL,
//                 totp_secret TEXT NOT NULL,
//                 jwt_secret TEXT NOT NULL,
//                 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
//                 updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
//                 last_login TIMESTAMPTZ,
//                 last_ip TEXT,
//                 addresses TEXT[] NOT NULL DEFAULT '{}',
//                 bound bool NOT NULL DEFAULT false
//             );
//         "#,
//         )
//         .execute(&self.pool)
//         .await?;

//         sqlx::query(
//             r#"CREATE UNIQUE INDEX IF NOT EXISTS users_username_idx ON users (LOWER(username));"#,
//         )
//         .execute(&self.pool)
//         .await?;

//         let first_user = match self.get_first_user().await {
//             Ok(user) => user,
//             Err(e) => {
//                 println!("Error getting first user: {}", e);
//                 let secret = generate_random_secret();
//                 self.create_user(DEFAULT_ADMIN_USERNAME.clone(), &secret)
//                     .await?
//             }
//         };

//         if !first_user.bound {
//             let res = get_totp_code(first_user.username, first_user.totp_secret)?;
//             println!(
//                 "The admin user is not binding the totp authenticator, please use the following totp to login: {}",
//                 res
//             );
//         }

//         Ok(())
//     }

//     async fn create_user(
//         &self,
//         username: impl Into<String>,
//         totp_secret: &str,
//     ) -> Result<DatabaseAuthentication> {
//         let username = username.into();
//         if self.is_exists_user(&username).await? {
//             return Err(anyhow::anyhow!("User already exists"));
//         }
//         sqlx::query(
//             r#"
//             INSERT INTO users (id, username, totp_secret, jwt_secret)
//             VALUES ($1, $2, $3, $4)
//         "#,
//         )
//         .bind(ObjectId::new())
//         .bind(&username)
//         .bind(totp_secret)
//         .bind(generate_random_secret())
//         .execute(&self.pool)
//         .await?;

//         self.get_user(&username).await
//     }

//     // async fn get_user_totp_secret(&self, username: String) -> Result<String> {
//     //     let totp_secret = sqlx::query_scalar(
//     //         r#"
//     //         SELECT totp_secret
//     //         FROM users
//     //         WHERE LOWER(username) = LOWER($1)
//     //     "#,
//     //     )
//     //     .bind(username)
//     //     .fetch_one(&self.pool)
//     //     .await?;

//     //     Ok(totp_secret)
//     // }

//     async fn is_exists_user(&self, username: &str) -> Result<bool> {
//         let exists = sqlx::query_scalar(
//             r#"
//             SELECT EXISTS (
//                 SELECT 1
//                 FROM users
//                 WHERE LOWER(username) = LOWER($1)
//             )
//         "#,
//         )
//         .bind(username)
//         .fetch_one(&self.pool)
//         .await?;

//         Ok(exists)
//     }

//     async fn get_user(&self, username: &str) -> Result<DatabaseAuthentication> {
//         let row = sqlx::query(r#"
//             SELECT id, username, totp_secret, jwt_secret, created_at, updated_at, last_login, last_ip, addresses, bound FROM users
//             where LOWER(username) = LOWER($1)
//         "#)
//         .bind(username)
//         .fetch_optional(&self.pool)
//         .await?;

//         let row = match row {
//             Some(row) => row,
//             None => {
//                 event!(Level::INFO, "User not found: {}", username);
//                 return Err(anyhow!("User not found"));
//             }
//         };

//         Ok(DatabaseAuthentication::from_row(&row)?)
//     }

//     async fn get_first_user(&self) -> Result<DatabaseAuthentication> {
//         let row = sqlx::query(r#"
//             SELECT id FROM users ORDER BY created_at ASC LIMIT 1
//         "#)
//         .fetch_one(&self.pool)
//         .await?;

//         let id = row.get::<ObjectId, _>("id");
//         self.get_user_from_id(&id).await
//     }

//     async fn verify_totp(&self, username: &str, totp: &str) -> Result<bool> {
//         let user = match self.get_user(username).await {
//             Ok(user) => user,
//             Err(_) => {
//                 return Ok(false);
//             }
//         };
//         let code = get_totp_code(username, user.totp_secret)?;
//         println!("Verifying TOTP code: {} == {}", totp, code);
//         Ok(totp.eq(code.as_str()))
//     }

//     async fn get_user_from_id(&self, user_id: &ObjectId) -> Result<DatabaseAuthentication> {
//         let row = sqlx::query(r#"
//             SELECT id, username, totp_secret, jwt_secret, created_at, updated_at, last_login, last_ip, addresses, bound FROM users
//             where id = $1
//         "#)
//         .bind(user_id)
//         .fetch_optional(&self.pool)
//         .await?;

//         let row = match row {
//             Some(row) => row,
//             None => {
//                 event!(Level::INFO, "User not found: {}", user_id);
//                 return Err(anyhow!("User not found"));
//             }
//         };

//         Ok(DatabaseAuthentication::from_row(&row)?)
//     }
// }

use std::net::SocketAddr;

use crate::{
    auth::{DEFAULT_ADMIN_USERNAME, generate_random_secret, get_totp_code},
    database::log::WebLogManager,
    models::auth::DatabaseAuthentication,
};
use anyhow::{Result, anyhow};
use shared::{
    database::{Database, get_database},
    objectid::ObjectId,
};
use sqlx::FromRow;
use tracing::{self, Level, event};

/// 用户表的所有列名，用于查询时复用
const USER_COLUMNS: &str = "id, username, totp_secret, jwt_secret, created_at, updated_at, last_login, last_ip, addresses, bound";

#[async_trait::async_trait]
pub trait Authentication {
    async fn create_user(
        &self,
        username: impl Into<String> + Send,
        totp_secret: &str,
    ) -> Result<DatabaseAuthentication>;
    async fn init_authentication(&self) -> Result<()>;
    // async fn is_exists_user(&self, username: &str) -> Result<bool>;
    async fn get_user(&self, username: &str) -> Result<DatabaseAuthentication>;
    async fn get_first_user(&self) -> Result<DatabaseAuthentication>;
    async fn verify_totp(&self, username: &str, totp: &str, addr: SocketAddr) -> Result<bool>;
    async fn get_user_from_id(&self, id: &ObjectId) -> Result<DatabaseAuthentication>;
}

#[async_trait::async_trait]
impl Authentication for Database {
    async fn init_authentication(&self) -> Result<()> {
        // 创建表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                totp_secret TEXT NOT NULL,
                jwt_secret TEXT NOT NULL,
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

        // 唯一索引
        sqlx::query(
            r#"CREATE UNIQUE INDEX IF NOT EXISTS users_username_idx ON users (LOWER(username));"#,
        )
        .execute(&self.pool)
        .await?;

        // 检查是否存在用户，若无则创建默认管理员
        let _ = match self.get_first_user().await {
            Ok(user) => user,
            Err(_) => {
                event!(Level::INFO, "No users found, creating default admin");
                let secret = generate_random_secret();
                self.create_user(DEFAULT_ADMIN_USERNAME.as_str(), &secret)
                    .await?
            }
        };

        Ok(())
    }

    async fn create_user(
        &self,
        username: impl Into<String> + Send,
        totp_secret: &str,
    ) -> Result<DatabaseAuthentication> {
        let username = username.into();
        let id = ObjectId::new();
        let jwt_secret = generate_random_secret();

        // 直接插入，依赖数据库唯一索引保证不重复
        let result = sqlx::query(
            r#"
            INSERT INTO users (id, username, totp_secret, jwt_secret)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(id)
        .bind(&username)
        .bind(totp_secret)
        .bind(&jwt_secret)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => self.get_user(&username).await,
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(anyhow!("User '{}' already exists", username))
            }
            Err(e) => Err(anyhow!("Failed to create user: {}", e)),
        }
    }

    async fn get_user(&self, username: &str) -> Result<DatabaseAuthentication> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {} FROM users
            WHERE LOWER(username) = LOWER($1)
            "#,
            USER_COLUMNS
        ))
        .bind(username)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("User '{}' not found", username))?;

        Ok(DatabaseAuthentication::from_row(&row)?)
    }

    async fn get_first_user(&self) -> Result<DatabaseAuthentication> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {} FROM users
            ORDER BY created_at ASC
            LIMIT 1
            "#,
            USER_COLUMNS
        ))
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("No users found"))?;

        Ok(DatabaseAuthentication::from_row(&row)?)
    }

    async fn verify_totp(&self, username: &str, totp: &str, addr: SocketAddr) -> Result<bool> {
        let user = match self.get_user(username).await {
            Ok(user) => user,
            Err(_) => return Ok(false), // 用户不存在视为验证失败
        };

        let expected = get_totp_code(username, user.totp_secret)?;
        let success = totp == expected;
        get_database()
            .add_web_log(
                &user.id,
                &crate::models::log::LogContent::Raw(format!(
                    "auth.user.login.{}",
                    if success { "success" } else { "fail" }
                )),
                &addr.to_string(),
            )
            .await?;
        Ok(totp == expected)
    }

    async fn get_user_from_id(&self, user_id: &ObjectId) -> Result<DatabaseAuthentication> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {} FROM users
            WHERE id = $1
            "#,
            USER_COLUMNS
        ))
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow!("User with id '{}' not found", user_id))?;

        Ok(DatabaseAuthentication::from_row(&row)?)
    }
}
