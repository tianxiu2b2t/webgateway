use axum::{Router, extract::Query, routing::get};
use shared::database::get_database;

use crate::{
    database::log::WebLogManager,
    models::{
        auth::AuthJWTInfoExtract,
        log::{Log, LogQueryParams},
    },
    response::APIResponse,
};

pub async fn info(AuthJWTInfoExtract(_): AuthJWTInfoExtract) -> APIResponse<usize> {
    APIResponse::result(get_database().get_web_logs_of_total().await)
}

pub async fn paged(
    AuthJWTInfoExtract(_): AuthJWTInfoExtract,
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
}
