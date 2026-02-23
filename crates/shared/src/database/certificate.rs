// use crate::database::Database;
// use crate::models::certificate::{
//     CreateDatabaseCertificate, DatabaseCertificate, UpdateDatabaseCertificate, extract_cert_info,
// };
// use crate::objectid::ObjectId;
// use anyhow::Result;
// use async_trait::async_trait;
// use chrono::{DateTime, Utc};

// #[async_trait]
// pub trait CertificateRepository: Send + Sync {
//     /// 创建新证书，自动计算 hostnames 和 expires_at
//     async fn create_certificate(&self, input: CreateDatabaseCertificate) -> Result<ObjectId>;

//     /// 更新证书（整体替换），自动重新计算 hostnames 和 expires_at
//     async fn update_certificate(&self, input: UpdateDatabaseCertificate) -> Result<()>;

//     /// 删除证书
//     async fn delete_certificate(&self, id: &ObjectId) -> Result<bool>;

//     /// 获取单个证书（完整记录）
//     async fn get_certificate(&self, id: &ObjectId) -> Result<Option<DatabaseCertificate>>;

//     /// 获取所有证书
//     async fn get_all_certificates(&self) -> Result<Vec<DatabaseCertificate>>;

//     /// 获取具有 DNS 提供商 ID 的证书（用于自动续期）
//     async fn get_certificates_with_dns_provider(&self) -> Result<Vec<DatabaseCertificate>>;

//     /// 获取即将过期的证书（在指定时间之前）
//     async fn get_expiring_certificates(
//         &self,
//         before: DateTime<Utc>,
//     ) -> Result<Vec<DatabaseCertificate>>;
// }

// #[async_trait]
// impl CertificateRepository for Database {
//     async fn create_certificate(&self, input: CreateDatabaseCertificate) -> Result<ObjectId> {
//         // 解析证书获取 hostnames 和 expires_at
//         let (hostnames, expires_at) = extract_cert_info(&input.fullchain)?;

//         let id = ObjectId::new();

//         sqlx::query(
//             r#"
//             INSERT INTO certificates (
//                 id, hostnames, fullchain, private_key,
//                 dns_provider_id, expires_at
//             )
//             VALUES ($1, $2, $3, $4, $5, $6)
//             "#,
//         )
//         .bind(id)
//         .bind(&hostnames)
//         .bind(&input.fullchain)
//         .bind(&input.private_key)
//         .bind(&input.dns_provider_id)
//         .bind(expires_at)
//         .execute(&self.pool)
//         .await?;

//         Ok(id)
//     }

//     async fn update_certificate(&self, input: UpdateDatabaseCertificate) -> Result<()> {
//         // 解析新证书内容
//         let (hostnames, expires_at) = extract_cert_info(&input.fullchain)?;

//         let rows_affected = sqlx::query(
//             r#"
//             UPDATE certificates
//             SET
//                 hostnames = $1,
//                 fullchain = $2,
//                 private_key = $3,
//                 dns_provider_id = $4,
//                 expires_at = $5,
//                 updated_at = NOW()
//             WHERE id = $6
//             "#,
//         )
//         .bind(&hostnames)
//         .bind(&input.fullchain)
//         .bind(&input.private_key)
//         .bind(&input.dns_provider_id)
//         .bind(expires_at)
//         .bind(input.id)
//         .execute(&self.pool)
//         .await?
//         .rows_affected();

//         if rows_affected == 0 {
//             anyhow::bail!("Certificate not found");
//         }

//         Ok(())
//     }

//     async fn delete_certificate(&self, id: &ObjectId) -> Result<bool> {
//         let rows = sqlx::query("DELETE FROM certificates WHERE id = $1")
//             .bind(id)
//             .execute(&self.pool)
//             .await?
//             .rows_affected();

//         Ok(rows > 0)
//     }

//     async fn get_certificate(&self, id: &ObjectId) -> Result<Option<DatabaseCertificate>> {
//         let cert = sqlx::query_as::<_, DatabaseCertificate>(
//             "SELECT id, hostnames, fullchain, private_key, dns_provider_id, expires_at, created_at, updated_at FROM certificates WHERE id = $1",
//         )
//         .bind(id)
//         .fetch_optional(&self.pool)
//         .await?;

//         Ok(cert)
//     }

//     async fn get_all_certificates(&self) -> Result<Vec<DatabaseCertificate>> {
//         let certs = sqlx::query_as::<_, DatabaseCertificate>(
//             "SELECT id, hostnames, fullchain, private_key, dns_provider_id, expires_at, created_at, updated_at FROM certificates",
//         )
//         .fetch_all(&self.pool)
//         .await?;

//         Ok(certs)
//     }

//     async fn get_certificates_with_dns_provider(&self) -> Result<Vec<DatabaseCertificate>> {
//         let certs = sqlx::query_as::<_, DatabaseCertificate>(
//             "SELECT id, hostnames, fullchain, private_key, dns_provider_id, expires_at, created_at, updated_at FROM certificates WHERE dns_provider_id IS NOT NULL",
//         )
//         .fetch_all(&self.pool)
//         .await?;

//         Ok(certs)
//     }

//     async fn get_expiring_certificates(
//         &self,
//         before: DateTime<Utc>,
//     ) -> Result<Vec<DatabaseCertificate>> {
//         let certs = sqlx::query_as::<_, DatabaseCertificate>(
//             "SELECT id, hostnames, fullchain, private_key, dns_provider_id, expires_at, created_at, updated_at FROM certificates WHERE expires_at <= $1",
//         )
//         .bind(before)
//         .fetch_all(&self.pool)
//         .await?;

//         Ok(certs)
//     }
// }

// pub trait InitDatabaseCertificate {
//     fn init_certificates(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
// }

// impl InitDatabaseCertificate for Database {
//     async fn init_certificates(&self) -> anyhow::Result<()> {
//         sqlx::query(
//             r#"
//             CREATE TABLE IF NOT EXISTS certificates (
//                 id TEXT PRIMARY KEY,
//                 hostnames TEXT[],
//                 fullchain TEXT,
//                 private_key TEXT,
//                 dns_provider_id TEXT,
//                 expires_at TIMESTAMPTZ,
//                 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
//                 updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
//             )
//             "#,
//         )
//         .execute(&self.pool)
//         .await?;

//         // 可选的索引
//         sqlx::query(
//             "CREATE INDEX IF NOT EXISTS idx_certificates_expires_at ON certificates(expires_at)",
//         )
//         .execute(&self.pool)
//         .await?;

//         Ok(())
//     }
// }

use anyhow::Result;
use async_trait::async_trait;

use crate::database::Database;

#[async_trait]
pub trait DatabaseCertificateInitializer {
    async fn initialize_certificates(&self) -> Result<()>;
}

#[async_trait]
impl DatabaseCertificateInitializer for Database {
    async fn initialize_certificates(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS certificates (
                id TEXT PRIMARY KEY,
                hostnames TEXT[],
                fullchain TEXT,
                private_key TEXT,
                dns_provider_id TEXT,
                expires_at TIMESTAMPTZ,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW,
        "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
