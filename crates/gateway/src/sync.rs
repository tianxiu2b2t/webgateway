use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use shared::database::{get_database, websites::DatabaseWebsiteQuery};
use tokio::sync::RwLock;
use tokio_schedule::Job;
use tracing::{Level, event};

// use crate::foundation::WebSiteRunner;

// pub static WEBSITE: LazyLock<Arc<RwLock<Vec<WebSiteRunner>>>> =
//     LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
// pub static LINKED_WEBSITES: LazyLock<Arc<RwLock<HashMap<String, WebSiteRunner>>>> =
//     LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

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
    // maybe need clean LINKED_WEBSITES, maybe make a lat performance
    Ok(())
}

pub async fn sync_websites() -> anyhow::Result<()> {
    let websites = get_database().get_websites().await?;
    Ok(())
}
