use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock as SyncRwLock},
    time::Duration,
};

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use shared::database::{get_database, websites::DatabaseWebsiteQuery};
use tokio::sync::RwLock;

use crate::state::WebSiteRunner;
static LAST_SYNC: LazyLock<RwLock<DateTime<Utc>>> =
    LazyLock::new(|| RwLock::new(DateTime::from_timestamp_secs(0).unwrap()));
static WEBSITES: LazyLock<DashMap<String, Arc<WebSiteRunner>>> = LazyLock::new(DashMap::default);
static CACHE_WEBSITES: LazyLock<SyncRwLock<ttl_cache::TtlCache<String, Arc<WebSiteRunner>>>> =
    LazyLock::new(|| SyncRwLock::new(ttl_cache::TtlCache::new((u16::MAX as usize) * 16)));
static CACHE_WEBSITES_EXPIRE: LazyLock<Arc<Duration>> =
    LazyLock::new(|| Arc::new(Duration::from_hours(2)));

pub async fn sync_websites() -> anyhow::Result<Vec<u16>> {
    let mut ports = vec![];
    let mut last_sync = { *LAST_SYNC.read().await };
    let websites = get_database()
        .get_websites_before_updated_at(&last_sync)
        .await?;
    for website in websites {
        let site = Arc::new(WebSiteRunner::new(website).await?);
        for domain in &site.inner().hosts {
            WEBSITES.insert(domain.to_owned(), site.clone());
        }
        for port in &site.inner().ports {
            if ports.contains(port) {
                continue;
            }
            ports.push(*port);
        }
        // compare
        if site.inner().updated_at > last_sync {
            last_sync = site.inner().updated_at;
        }
    }
    *LAST_SYNC.write().await = last_sync;
    Ok(ports)
}

pub async fn get_website(domain: &str) -> Option<Arc<WebSiteRunner>> {
    WEBSITES.get(domain).map(|x| x.value().clone())
}
