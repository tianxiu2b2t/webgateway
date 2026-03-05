use std::{collections::HashMap, sync::{Arc, LazyLock}};

use chrono::{DateTime, Utc};
use shared::{database::{get_database, websites::DatabaseWebsiteQuery}, models::websites::DatabaseWebsite};
use tokio::sync::RwLock;
static LAST_SYNC: LazyLock<RwLock<DateTime<Utc>>> = LazyLock::new(|| RwLock::new(DateTime::from_timestamp_secs(0).unwrap()));
static WEBSITES: LazyLock<RwLock<HashMap<String, Arc<DatabaseWebsite>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn sync_websites() -> anyhow::Result<Vec<u16>> {
    let mut ports = vec![];
    let mut last_sync = {
        *LAST_SYNC.read().await
    };
    let websites = get_database().get_websites_before_updated_at(&last_sync).await?;
    for website in websites {
        let site = Arc::new(website);
        for domain in &site.hosts {
            WEBSITES.write().await.insert(domain.to_owned(), site.clone());
        }
        for port in &site.ports {
            if ports.contains(port) {
                continue;
            }
            ports.push(*port);
        }
        // compare
        if site.updated_at > last_sync {
            last_sync = site.updated_at;
        }
    }
    *LAST_SYNC.write().await = last_sync;
    Ok(ports)
}