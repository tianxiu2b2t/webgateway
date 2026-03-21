use axum::{Router, extract::Query, middleware, routing::get};
use shared::{
    database::{access::DatabaseAccessLogsRepository, get_database},
    models::access::{AccessInfo, QueryAccessInfo, QueryQPS, QueryQPSType, ResponseQPS, TodayMetricsInfoOfWebsite},
};

use crate::{auth::middle_refresh_token, response::APIResponse};

pub async fn qps(Query(query): Query<QueryQPS>) -> APIResponse<ResponseQPS> {
    APIResponse::result(match query.interval {
        QueryQPSType::Second => get_database().get_qps_per_second(query.count).await,
        QueryQPSType::FiveSeconds => get_database().get_qps_per_5s(query.count).await,
    })
}

pub async fn access_info(Query(query): Query<QueryAccessInfo>) -> APIResponse<AccessInfo> {
    APIResponse::result(get_database().get_access_info(query.in_days.into()).await)
}

pub async fn website_metrics_info(
) -> APIResponse<Vec<TodayMetricsInfoOfWebsite>> {
    APIResponse::result(get_database().get_today_metrics_info_of_websites().await)
}

pub fn router() -> Router {
    Router::new()
        .route("/qps", get(qps))
        .route("/info", get(access_info))
        .route("/metrics/websites", get(website_metrics_info))
        .layer(middleware::from_fn(middle_refresh_token))
}
