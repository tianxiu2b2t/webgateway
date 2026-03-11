use std::sync::{Arc, LazyLock};

use rustls::ServerConfig;
use tokio_schedule::Job;
use tracing::{Level, event};

use crate::{
    proxy::listen,
    sync::{
        cert::{AutoCertificate, sync_certificates},
        websites::sync_websites,
    },
};

pub mod cert;
pub mod websites;

pub static SERVER_CONFIG: LazyLock<Arc<ServerConfig>> = LazyLock::new(|| {
    Arc::new(
        {
            let mut config = ServerConfig::builder()
                .with_no_client_auth()
                .with_cert_resolver(Arc::new(AutoCertificate));
            // config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
            config
        },
    )
});

pub async fn main() -> anyhow::Result<()> {
    let first_result = sync_config().await;
    if let Err(e) = first_result {
        event!(Level::ERROR, "Failed to sync first config: {e}");
        return Err(e);
    }
    tokio::spawn(tokio_schedule::every(10).seconds().perform(|| async {
        if let Err(e) = sync_config().await {
            event!(Level::ERROR, "Failed to sync config: {e}");
        }
    }));
    Ok(())
}

pub async fn sync_config() -> anyhow::Result<()> {
    event!(Level::DEBUG, "Syncing config at {}", chrono::Local::now());
    event!(Level::DEBUG, "Syncing certificates");
    sync_certificates().await?;
    event!(Level::DEBUG, "Syncing websites");
    let ports = sync_websites().await?;
    for port in ports {
        listen(port).await?;
    }
    // maybe need clean LINKED_WEBSITES, maybe make a lat performance
    Ok(())
}
