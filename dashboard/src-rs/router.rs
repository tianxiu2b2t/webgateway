use axum::Router;

pub mod dnsprovider;
pub mod log;
pub mod website;

pub fn get_router() -> Router {
    Router::new()
        .nest("/websites", website::router())
        .nest("/logs", log::router())
        .nest("/dnsproviders", dnsprovider::router())
}
