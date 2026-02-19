use axum::{Router, routing::get};
use shared::{
    database::{get_database, websites::DatabaseWebsiteQuery},
    models::websites::DatabaseWebsite,
};

use crate::response::APIResponse;

pub async fn get_all() -> APIResponse<Vec<DatabaseWebsite>> {
    APIResponse::ok(get_database().get_websites().await.ok().unwrap_or(vec![]))
}

pub fn router() -> Router {
    Router::new().route("/", get(get_all))
}
