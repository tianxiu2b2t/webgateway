use std::{
    sync::{Arc, LazyLock, RwLock as SyncRwLock},
    time::Duration,
};

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use regex::Regex;
use shared::database::{get_database, websites::DatabaseWebsiteQuery};
use tokio::sync::RwLock;
use tracing::event;

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
            if !WEBSITES.contains_key(domain) {
                event!(tracing::Level::INFO, "Sync website: {:?} => {:?}", domain, site.inner().backends);
            }
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
    // 1. 先尝试从缓存中获取（读锁）
    {
        let cache = CACHE_WEBSITES.read().unwrap();
        if let Some(cached) = cache.get(domain) {
            return Some(cached.clone());
        }
    } // 读锁在此释放

    // 2. 精确匹配：直接从 WEBSITES 中查找
    if let Some(entry) = WEBSITES.get(domain) {
        let site = entry.value().clone();
        // 插入缓存（写锁，double-check 避免重复插入）
        let mut cache = CACHE_WEBSITES.write().unwrap();
        if !cache.contains_key(domain) {
            cache.insert(domain.to_string(), site.clone(), **CACHE_WEBSITES_EXPIRE);
        }
        return Some(site);
    }

    // 3. 通配符匹配：为了避免长时间持有 DashMap 的锁，先收集所有键值对
    let entries: Vec<(String, Arc<WebSiteRunner>)> = WEBSITES
        .iter()
        .map(|entry| (entry.key().clone(), entry.value().clone()))
        .collect();

    for (pattern, site) in entries {
        // 支持 "*" 和含有 * 的模式
        if pattern == "*" || regex_match(domain, &pattern) {
            // 插入缓存（写锁，double-check）
            let mut cache = CACHE_WEBSITES.write().unwrap();
            if !cache.contains_key(domain) {
                cache.insert(domain.to_string(), site.clone(), **CACHE_WEBSITES_EXPIRE);
            }
            return Some(site);
        }
    }

    None
}

fn regex_match(host: &str, pattern: &str) -> bool {
    // 转义点号，将 * 替换为正则中的通配符（允许字母、数字、连字符）
    let pattern = pattern.replace('.', "\\.").replace('*', r"[-\w]+");
    // 编译正则，如果失败则返回 false（一般不会发生）
    let Ok(re) = Regex::new(&format!("^{}$", pattern)) else {
        return false;
    };
    re.is_match(host)
}
