use std::{net::{IpAddr, SocketAddr}, sync::Arc};

use anyhow::anyhow;
use protocols::tls::ProtocolTLS;
use shared::models::websites::DatabaseWebsite;
use tokio::net::lookup_host;

use crate::proxy::backends::{BackendConnectionPool, BackendConnectionPoolConfig};

#[derive(Debug)]
pub struct WebSiteRunner {
    inner: DatabaseWebsite,
    pool: Arc<BackendConnectionPool>,
}

impl WebSiteRunner {
    pub async fn new(inner: DatabaseWebsite) -> anyhow::Result<Self> {
        // only get first
        // TODO
        println!("Found {:?} backends", inner.backends);
        let backend = inner
            .backends
            .first()
            .ok_or(anyhow!("No found any backends"))?;
        let hostname = backend.url.host_str().ok_or(anyhow!("No found any host"))?;
        println!("Found host: {hostname}");
        // dns resolver it
        let addrs = lookup_host(format!(
            "{hostname}:{}",
            backend.url.port_or_known_default().unwrap_or(80)
        ))
        .await?
        .collect::<Vec<SocketAddr>>();
        Ok(Self {
            inner,
            pool: BackendConnectionPool::new(BackendConnectionPoolConfig::new_from_targets(addrs)),
        })
    }

    pub fn inner(&self) -> &DatabaseWebsite {
        &self.inner
    }

    pub fn pool(&self) -> &Arc<BackendConnectionPool> {
        &self.pool
    }
}

#[derive(Debug, Clone)]
pub struct ClientState {
    pub tls: Option<ProtocolTLS>,
    pub remote_addr: IpAddr,
}
