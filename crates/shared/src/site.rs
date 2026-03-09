use url::Url;
mod favicon;

pub async fn get_favicon(site: Url) -> Option<Vec<u8>> {
    let res = favicon::get_favicon_from_default(&site).await;
    match res {
        Some(res) => Some(res),
        None => favicon::get_favicon_from_index(&site).await,
    }
}
