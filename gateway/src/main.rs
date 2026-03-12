use shared::{
    database,
    logger::{self, LoggerConfig},
};
use tokio::signal::ctrl_c;

use crate::config::get_config;

pub mod config;
pub mod dns;
pub mod foundation;
pub mod proxy;
pub mod response;
pub mod state;
pub mod sync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init(LoggerConfig::default());
    config::init_config()?;
    database::init_database(&get_config().database, get_config().max_connections).await?;
    // ...
    // need wait for sync and then start proxy, so it need wait for sync config state;
    sync::main().await?;
    match ctrl_c().await {
        Ok(()) => println!("Ctrl-C received, shutting down..."),
        Err(err) => println!("Error waiting for ctrl-c: {:?}", err),
    }

    Ok(())
}
