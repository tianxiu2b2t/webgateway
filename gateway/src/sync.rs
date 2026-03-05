use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use rustls::ServerConfig;
use shared::database::{certificate::DatabaseCertificateRepository, get_database, websites::DatabaseWebsiteQuery};
use tokio::sync::RwLock;
use tokio_schedule::Job;
use tracing::{Level, event};

// use crate::foundation::WebSiteRunner;

// pub static WEBSITE: LazyLock<Arc<RwLock<Vec<WebSiteRunner>>>> =
//     LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
// pub static LINKED_WEBSITES: LazyLock<Arc<RwLock<HashMap<String, WebSiteRunner>>>> =
//     LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

pub static CERTIFICATES: LazyLock<RwLock<HashMap<String, Arc<ServerConfig>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

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
    sync_certificates().await?;
    // maybe need clean LINKED_WEBSITES, maybe make a lat performance
    Ok(())
}

pub async fn sync_websites() -> anyhow::Result<()> {
    let websites = get_database().get_websites().await?;
    Ok(())
}

pub async fn sync_certificates() -> anyhow::Result<()> {
    let certificates = get_database().get_certificates().await?;
    for certificate in certificates {
        let config = Arc::new(ServerConfig::builder().with_no_client_auth().with_single_cert(certificate.get_fullchain()?, certificate.get_private_key()?)?);
        let domains = certificate.hostnames;
        for domain in domains {
            CERTIFICATES.write().await.insert(domain, config.clone());
        }
    }
    Ok(())
}
