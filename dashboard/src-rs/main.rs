use crate::{
    config::{get_config, init_config},
    database::{auth::Authentication, log::WebLogInitializer},
    foundation::{CListener, RemoteAddr},
    response::wrapper_router,
};
use shared::{
    database::{get_database, init_database},
    listener::CustomDualStackTcpListener,
    logger::LoggerConfig,
};
use tokio::signal::ctrl_c;
use tracing::{Level, event};

pub mod auth;
pub mod certificate;
pub mod config;
mod database;
mod foundation;
pub mod mnt;
pub mod models;
pub mod response;
pub mod router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LoggerConfig::default().init();
    init_config()?;
    init_database(&get_config().database, get_config().max_connections).await?;
    get_database().init_authentication().await?;
    get_database().initialize_web_log().await?;

    event!(
        Level::INFO,
        "Dashboard API listening on port {}",
        get_config().port
    );
    let listener = CustomDualStackTcpListener::new_by_port(get_config().port).await?;
    let router = axum::Router::new()
        .nest("/auth", auth::get_router())
        .merge(router::get_router());

    let web = tokio::spawn(async move {
        let r = axum::serve(
            CListener::from(listener),
            wrapper_router(router).into_make_service_with_connect_info::<RemoteAddr>(),
        )
        .await;
        if let Err(e) = r {
            event!(Level::ERROR, "Error while serving: {}", e);
        }
    });

    let mnt = tokio::spawn(async move {
        // unix for mgt
        let res = mnt::init().await;
        if let Err(e) = res {
            event!(Level::ERROR, "Error while serving: {}", e);
        }
    });

    let auto_cert = tokio::spawn(async move {
        let res = certificate::init().await;
        if let Err(e) = res {
            event!(Level::ERROR, "Error while serving: {}", e);
        }
    });

    match ctrl_c().await {
        Ok(()) => {
            web.abort();
            mnt.abort();
            auto_cert.abort();
            event!(Level::INFO, "Dashboard API shutting down")
        }
        Err(e) => event!(Level::ERROR, "Dashboard API failed to shut down: {}", e),
    };

    Ok(())
}
