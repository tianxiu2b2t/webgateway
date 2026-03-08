use url::Url;

use crate::default::reqwest_default_client;

pub async fn get_favicon_from_default(url: &Url) -> Option<Vec<u8>> {
    // TODO: Implement favicon fetching
    let favicon = url.join("./favicon.ico").ok()?;
    Some(reqwest_default_client().get(favicon).send().await.ok()?.bytes().await.ok()?.to_vec())
}

pub async fn get_favicon_from_index(url: &Url) -> Option<Vec<u8>> {
    // TODO: Implement favicon fetching
    let index = url.join("./").ok()?;
    let content = reqwest_default_client().get(index).send().await.ok()?.text().await.ok()?;
    for line in content.lines() {
        if line.starts_with("<link rel=\"icon\"") {
            let link = line.split('\"').nth(1)?;
            let favicon = url::Url::options().base_url(Some(url)).parse(link).ok()?;
            return Some(reqwest_default_client().get(favicon).send().await.ok()?.bytes().await.ok()?.to_vec());
        }
    }
    None
}

