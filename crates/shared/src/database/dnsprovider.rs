use async_trait::async_trait;

use crate::{database::Database, models::dnsprovider::DatabaseDNSProvider};
use anyhow::Result;

#[async_trait]
pub trait DatabaseDNSProviderInitializer {
    async fn initialize_dns_provider(&self) -> Result<()>;
}

#[async_trait]
impl DatabaseDNSProviderInitializer for Database {
    async fn initialize_dns_provider(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS dns_providers (
                id TEXT PRIMARY KEY,
                provider_type TEXT NOT NULL,
                provider_config JSONB NOT NULL,
                domains TEXT[] NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW
        "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
pub trait DatabaseDNSProviderQuery {
    async fn get_dns_providers(&self) -> Result<Vec<DatabaseDNSProvider>>;
}

#[async_trait]
impl DatabaseDNSProviderQuery for Database {
    async fn get_dns_providers(&self) -> Result<Vec<DatabaseDNSProvider>> {
        let rows = sqlx::query_as::<_, DatabaseDNSProvider>(
            r#"
            SELECT * FROM dns_providers
        "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}

