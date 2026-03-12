use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::{
    database::Database,
    models::certificate::{
        CreateCertificate, CreateCertificateMethod, DatabaseCertificate, NeedSignCertificate,
        UpdateCertificate,
    },
    objectid::ObjectId,
};

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
                name TEXT,
                hostnames TEXT[],
                fullchain TEXT,
                private_key TEXT,
                dns_provider_id TEXT,
                email TEXT,
                expires_at TIMESTAMPTZ,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                FOREIGN KEY (dns_provider_id) REFERENCES dns_providers(id) ON DELETE SET NULL
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        self.create_trigger_notify("certificates").await?;

        Ok(())
    }
}

#[async_trait]
pub trait DatabaseCertificateRepository {
    async fn get_certificates(&self) -> Result<Vec<DatabaseCertificate>>;
    async fn get_certificates_before_updated_at(
        &self,
        before: &chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<DatabaseCertificate>>;
    async fn get_will_sign_certificates(&self) -> Result<Vec<NeedSignCertificate>>;

    async fn update_certificate(&self, cert: &UpdateCertificate) -> Result<()>;
    async fn get_total_of_certificates(&self) -> Result<usize>;
    async fn get_certificates_by_page(
        &self,
        page: usize,
        limit: usize,
    ) -> Result<Vec<DatabaseCertificate>>;

    async fn create_certificate(&self, cert: &CreateCertificate) -> Result<DatabaseCertificate>;
}

#[async_trait]
impl DatabaseCertificateRepository for Database {
    async fn get_certificates(&self) -> Result<Vec<DatabaseCertificate>> {
        let certs = sqlx::query_as::<_, DatabaseCertificate>(
            "SELECT id, name, hostnames, fullchain, private_key, dns_provider_id, email, expires_at, created_at, updated_at FROM certificates",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(certs)
    }

    async fn get_certificates_before_updated_at(
        &self,
        before: &DateTime<Utc>,
    ) -> Result<Vec<DatabaseCertificate>> {
        let certs = sqlx::query_as::<_, DatabaseCertificate>
            ("SELECT id, name, hostnames, fullchain, private_key, dns_provider_id, email, expires_at, created_at, updated_at FROM certificates WHERE updated_at > $1")
            .bind(before)
            .fetch_all(&self.pool)
            .await?;
        Ok(certs)
    }

    async fn get_will_sign_certificates(&self) -> Result<Vec<NeedSignCertificate>> {
        let certs = sqlx::query_as::<_, NeedSignCertificate>(
            "SELECT id, name, hostnames, dns_provider_id FROM certificates WHERE (expires_at IS NULL OR expires_at < (NOW() - '7 days'::INTERVAL)) AND dns_provider_id IS NOT NULL AND email IS NOT NULL",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(certs)
    }

    async fn update_certificate(&self, cert: &UpdateCertificate) -> Result<()> {
        sqlx::query(
            "UPDATE certificates SET fullchain = $1, private_key = $2, expires_at = $3, updated_at = NOW() WHERE id = $4",
        )
        .bind(&cert.fullchain)
        .bind(&cert.private_key)
        .bind(cert.expires_at()?)
        .bind(cert.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_total_of_certificates(&self) -> Result<usize> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(id) FROM certificates")
            .fetch_one(&self.pool)
            .await?;

        Ok(count as usize)
    }

    async fn get_certificates_by_page(
        &self,
        page: usize,
        limit: usize,
    ) -> Result<Vec<DatabaseCertificate>> {
        let certificates: Vec<DatabaseCertificate> = sqlx::query_as(
            r#"SELECT * FROM certificates ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
        )
        .bind(limit as i64)
        .bind((std::cmp::max(0, page) * limit) as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(certificates)
    }

    async fn create_certificate(&self, cert: &CreateCertificate) -> Result<DatabaseCertificate> {
        let res = match &cert.content {
            CreateCertificateMethod::AUTO(context) => {
                sqlx::query_as::<_, DatabaseCertificate>(
                    r#"
                    INSERT INTO certificates (id, name, hostnames, dns_provider_id, email) VALUES ($1, $2, $3, $4, $5)
                    RETURNING *
                "#,
                )
                .bind(ObjectId::new())
                .bind(&cert.name)
                .bind(&context.hostnames)
                .bind(context.dns_provider_id)
                .bind(&context.email)
                .fetch_one(&self.pool)
                .await?
            },
            CreateCertificateMethod::MANUAL(context) => {
                println!("test");
                sqlx::query_as::<_, DatabaseCertificate>(
                    r#"
                    INSERT INTO certificates (id, name, hostnames, fullchain, private_key, expires_at) VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING *
                "#,
                )
                .bind(ObjectId::new())
                .bind(&cert.name)
                .bind(&context.hostnames()?)
                .bind(&context.fullchain)
                .bind(&context.private_key)
                .bind(context.expires_at()?)
                .fetch_one(&self.pool)
                .await?
            }
        };
        Ok(res)
    }
}
