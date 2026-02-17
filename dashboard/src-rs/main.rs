use crate::{
    config::{get_config, init_config},
    database::auth::Authentication,
    foundation::CListener,
    response::wrapper_router,
};
use axum::routing::get;
use shared::{
    database::{get_database, init_database},
    listener::CustomDualStackTcpListener,
    logger::LoggerConfig,
};
use tracing::{Level, event};

pub mod auth;
pub mod config;
mod database;
mod foundation;
pub mod models;
pub mod response;

async fn index() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LoggerConfig::default().init();
    init_config()?;
    init_database(&get_config().database, get_config().max_connections).await?;
    get_database().init_authentication().await?;

    event!(
        Level::INFO,
        "Dashboard API listening on port {}",
        get_config().port
    );
    let listener = CustomDualStackTcpListener::new_by_port(get_config().port).await?;
    let router = axum::Router::new()
        .route("/", get(index))
        .nest("/auth", auth::get_router());

    axum::serve(CListener::from(listener), wrapper_router(router)).await?;

    Ok(())
}
