use shared::logger::{self, LoggerConfig};
use tokio::signal::ctrl_c;

pub mod foundation;
pub mod proxy;
pub mod state;
pub mod sync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init(LoggerConfig::default());
    // ...
    // need wait for sync and then start proxy, so it need wait for sync config state;
    sync::main().await?;
    match ctrl_c().await {
        Ok(()) => println!("Ctrl-C received, shutting down..."),
        Err(err) => println!("Error waiting for ctrl-c: {:?}", err),
    }

    Ok(())
}
