use axum::{Json, Router, routing::{get, post}};
use shared::{
    database::{get_database, websites::{DatabaseWebsiteQuery, DatabaseWebsiteSet}},
    models::websites::{CreateDatabaseWebsite, DatabaseWebsite},
};

use crate::{models::auth::AuthJWTInfoExtract, response::APIResponse};

pub async fn get_all() -> APIResponse<Vec<DatabaseWebsite>> {
    APIResponse::ok(get_database().get_websites().await.ok().unwrap_or(vec![]))
}

pub async fn create(
    AuthJWTInfoExtract(_): AuthJWTInfoExtract,
    Json(data): Json<CreateDatabaseWebsite>,
) -> APIResponse<DatabaseWebsite> {
    APIResponse::result(get_database().create_website(&data).await)
}

pub fn router() -> Router {
    Router::new().route("/", get(get_all))
        .route("/create", post(create))
}
