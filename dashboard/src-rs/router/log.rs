use axum::{Router, extract::Query, middleware, routing::get};
use shared::database::get_database;

use crate::{
    auth::middle_refresh_token,
    database::log::WebLogManager,
    models::{
        log::{Log, LogQueryParams},
    },
    response::APIResponse,
};

pub async fn info() -> APIResponse<usize> {
    APIResponse::result(get_database().get_total_of_web_logs().await)
}

pub async fn paged(
    Query(query): Query<LogQueryParams>,
) -> APIResponse<Vec<Log>> {
    APIResponse::result(
        get_database()
            .get_web_logs_by_page(
                query.page.unwrap_or(0),
                std::cmp::min(query.limit.unwrap_or(20), 100),
            )
            .await,
    )
}

pub fn router() -> Router {
    Router::new()
        .route("/total", get(info))
        .route("/page", get(paged))
        .layer(middleware::from_fn(middle_refresh_token))
}
