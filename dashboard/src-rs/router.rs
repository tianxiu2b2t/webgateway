use axum::Router;

pub mod website;

pub fn get_router() -> Router {
    Router::new().nest("/website", website::router())
}
