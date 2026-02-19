use sqlx::{FromRow, types::Json};
use sqlx_pg_ext_uint::c_u16::U16;

use crate::{
    database::Database,
    models::websites::{CreateDatabaseWebsite, DatabaseWebsite, DatabaseWebsiteConfig},
    objectid::ObjectId,
};

#[async_trait::async_trait]
pub trait DatabaseWebsiteInitializer {
    async fn initialize_websites(&self) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl DatabaseWebsiteInitializer for Database {
    async fn initialize_websites(&self) -> anyhow::Result<()> {
        for sql in [
            r#"CREATE TABLE IF NOT EXISTS websites (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                hosts TEXT[] NOT NULL DEFAULT '{}',
                ports uint2[] NOT NULL DEFAULT '{}',
                certificates TEXT[] NOT NULL DEFAULT '{}',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                backends JSONB NOT NULL,
                config JSONB NOT NULL
            );"#,
            "CREATE INDEX IF NOT EXISTS idx_websites_hosts ON websites USING GIN (hosts);",
            "CREATE INDEX IF NOT EXISTS idx_websites_name ON websites USING GIN (name);",
            "CREATE INDEX IF NOT EXISTS idx_websites_created_at ON websites (created_at);",
        ] {
            sqlx::query(sql).execute(&self.pool).await?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait DatabaseWebsiteQuery {
    async fn get_websites(&self) -> anyhow::Result<Vec<DatabaseWebsite>>;
    async fn get_website(&self, id: &ObjectId) -> anyhow::Result<DatabaseWebsite>;
}

#[async_trait::async_trait]
impl DatabaseWebsiteQuery for Database {
    async fn get_websites(&self) -> anyhow::Result<Vec<DatabaseWebsite>> {
        let rows = sqlx::query("SELECT * FROM websites;")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows
            .iter()
            .flat_map(|v| DatabaseWebsite::from_row(v).ok())
            .collect::<Vec<DatabaseWebsite>>())
    }

    async fn get_website(&self, id: &ObjectId) -> anyhow::Result<DatabaseWebsite> {
        let row = sqlx::query("SELECT * FROM websites WHERE id = $1;")
            .bind(id.to_string())
            .fetch_one(&self.pool)
            .await?;
        Ok(DatabaseWebsite::from_row(&row)?)
    }
}

#[async_trait::async_trait]
pub trait DatabaseWebsiteSet {
    async fn create_website(
        &self,
        website: &CreateDatabaseWebsite,
    ) -> anyhow::Result<DatabaseWebsite>;
}

#[async_trait::async_trait]
impl DatabaseWebsiteSet for Database {
    async fn create_website(
        &self,
        website: &CreateDatabaseWebsite,
    ) -> anyhow::Result<DatabaseWebsite> {
        let id = ObjectId::new();
        let row = sqlx::query("INSERT INTO websites (id, name, hosts, ports, certificates, backends, config) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;")
            .bind(id)
            .bind(website.name.to_string())
            .bind(website.hosts.to_vec())
            .bind(website.ports.to_vec().iter().map(|v| U16::from(*v)).collect::<Vec<U16>>())
            .bind(website.certificates.to_vec())
            .bind(Json(&website.backends.to_vec()))
            .bind(Json(website.config.as_ref().unwrap_or(&DatabaseWebsiteConfig::default())))
            .fetch_one(&self.pool)
            .await?;
        Ok(DatabaseWebsite::from_row(&row)?)
    }
}
