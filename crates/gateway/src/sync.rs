use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use tokio::sync::RwLock;

use crate::foundation::WebSiteRunner;

pub static WEBSITE: LazyLock<Arc<RwLock<Vec<WebSiteRunner>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
pub static LINKED_WEBSITES: LazyLock<Arc<RwLock<HashMap<String, WebSiteRunner>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

pub async fn main() {}

pub async fn sync_config() {
    // maybe need clean LINKED_WEBSITES, maybe make a lat performance
}
