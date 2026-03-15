use axum::{Router, extract::Query, middleware, routing::get};
use shared::{database::{access::DatabaseAccessLogsRepository, get_database}, models::access::{AccessInfo, QueryAccessInfo, QueryQPS, QueryQPSType, ResponseQPS}};

use crate::{
    auth::middle_refresh_token,
    response::APIResponse,
};

pub async fn qps(
    Query(query): Query<QueryQPS>
) -> APIResponse<ResponseQPS> {
    APIResponse::result(match query.interval {
        QueryQPSType::Second => get_database().get_qps_per_second().await,
        QueryQPSType::FiveSeconds => get_database().get_qps_per_5s().await,
    })
}

pub async fn access_info(
    Query(query): Query<QueryAccessInfo>
) -> APIResponse<AccessInfo> {
    APIResponse::result(get_database().get_access_info(query.in_days.into()).await)
}

pub fn router() -> Router {
    Router::new()
        .route("/qps", get(qps))
        .route("/info", get(access_info))
        .layer(middleware::from_fn(middle_refresh_token))
}
