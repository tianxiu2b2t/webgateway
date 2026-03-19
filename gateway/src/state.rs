use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use anyhow::anyhow;
use protocols::tls::ProtocolTLS;
use shared::{models::websites::DatabaseWebsite, objectid::ObjectId};
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
        let backend = inner
            .backends
            .first()
            .ok_or(anyhow!("No found any backends"))?;
        let hostname = backend.url.host_str().ok_or(anyhow!("No found any host"))?;
        // dns resolver it
        let addrs = lookup_host(format!(
            "{hostname}:{}",
            backend.url.port_or_known_default().unwrap_or(80)
        ))
        .await?
        .collect::<Vec<SocketAddr>>();
        let url = backend.url.clone();
        Ok(Self {
            inner,
            pool: BackendConnectionPool::new(
                BackendConnectionPoolConfig::new_from_targets(addrs).url(url),
            ),
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
pub struct BaseClientState {
    pub tls: Option<ProtocolTLS>,
    pub remote_addr: IpAddr,
    pub local_addr: IpAddr,
}

#[derive(Debug, Clone)]
pub struct ClientState {
    pub base: Arc<BaseClientState>,
    pub website: Arc<WebSiteRunner>,
    pub host: String,
    pub id: ObjectId,
}

impl ClientState {
    pub fn new(
        base: Arc<BaseClientState>,
        website: Arc<WebSiteRunner>,
        host: String,
        id: &ObjectId,
    ) -> Self {
        Self {
            base,
            website,
            host,
            id: *id,
        }
    }

    pub fn tls(&self) -> Option<&ProtocolTLS> {
        self.base.tls.as_ref()
    }
    pub fn remote_addr(&self) -> IpAddr {
        self.base.remote_addr
    }
    pub fn local_addr(&self) -> IpAddr {
        self.base.local_addr
    }
    pub fn scheme(&self) -> &str {
        if self.tls().is_some() {
            "https"
        } else {
            "http"
        }
    }
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
}
