use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use shared::default::default_database_max_connections;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainConfig {
    pub database: String,
    #[serde(
        default = "default_database_max_connections",
        rename = "database_max_connections"
    )]
    pub max_connections: u32,
}

pub static CONFIG: OnceLock<MainConfig> = OnceLock::new();

pub fn init_config() -> anyhow::Result<()> {
    // laod from toml
    let content = std::fs::read_to_string("./config.toml")?;
    let config = toml::from_str::<MainConfig>(&content)?;
    CONFIG.set(config).unwrap();

    Ok(())
}

pub fn get_config() -> &'static MainConfig {
    CONFIG.get().unwrap()
}
