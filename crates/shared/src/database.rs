use std::sync::OnceLock;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::database::{certificate::InitDatabaseCertificate, websites::DatabaseWebsiteInitializer};

pub mod certificate;
pub mod dnsprovider;
pub mod websites;

#[derive(Debug)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }
}

pub static DATABASE: OnceLock<Database> = OnceLock::new();

pub async fn init_database(url: &str, max_connections: u32) -> anyhow::Result<()> {
    let database = Database::new(url, max_connections).await?;
    DATABASE.set(database).unwrap();

    inner_init_database().await?;

    Ok(())
}

pub fn get_database() -> &'static Database {
    DATABASE.get().unwrap()
}

async fn inner_init_database() -> anyhow::Result<()> {
    get_database().init_certificates().await?;
    get_database().initialize_websites().await?;
    Ok(())
}
