use std::{sync::{Arc, LazyLock, RwLock as SyncRwLock}, time::Duration};

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use regex::Regex;
use rustls::{ServerConfig, crypto::CryptoProvider, server::ResolvesServerCert, sign::CertifiedKey};
use shared::{database::{certificate::DatabaseCertificateRepository, get_database}, default::sign_default_certificates};
use tokio::sync::RwLock;
pub static CERTIFICATES: LazyLock<DashMap<String, Arc<rustls::sign::CertifiedKey>>> = LazyLock::new(DashMap::default);
static DEFAULT_CERTIFICATE: LazyLock<Arc<CertifiedKey>> = LazyLock::new(|| {
    let (fullchain, privatekey) = sign_default_certificates().unwrap();
    Arc::new(CertifiedKey::from_der(fullchain, privatekey, &PROVIDER).unwrap())
});
static PROVIDER: LazyLock<Arc<CryptoProvider>> = LazyLock::new(|| ServerConfig::builder().crypto_provider().clone());
static LAST_SYNC: LazyLock<RwLock<DateTime<Utc>>> = LazyLock::new(|| RwLock::new(DateTime::from_timestamp_secs(0).unwrap()));
static CACHE_CERTIFICATES: LazyLock<SyncRwLock<ttl_cache::TtlCache<String, Arc<rustls::sign::CertifiedKey>>>> = LazyLock::new(|| {
    SyncRwLock::new(ttl_cache::TtlCache::new((u16::MAX as usize) * 16))
});
static CACHE_CERTIFICATES_EXPIRE: LazyLock<Arc<Duration>> = LazyLock::new(|| Arc::new(Duration::from_hours(2)));

#[derive(Debug, Default)]
pub struct AutoCertificate;

impl ResolvesServerCert for AutoCertificate {
    fn resolve(&self, client_hello: rustls::server::ClientHello<'_>) -> Option<Arc<rustls::sign::CertifiedKey>> {
        // TODO: implement
        let sni = client_hello.server_name();
        if let Some(sni) = sni && let Some(cert) = lookup_certificate(&sni.to_lowercase()) {
            return Some(cert);
        }
        DEFAULT_CERTIFICATE.clone().into()
    }
}

pub async fn sync_certificates() -> anyhow::Result<()> {
    let mut last_sync = {
        *LAST_SYNC.read().await
    };
    let certificates = get_database().get_certificates_before_updated_at(&last_sync).await?;
    for certificate in certificates {
        let fullchain = certificate.get_fullchain()?;
        let privatekey = certificate.get_private_key()?;
        let config = Arc::new(CertifiedKey::from_der(fullchain, privatekey, &PROVIDER)?);
        let domains = certificate.hostnames;
        for domain in domains {
            CERTIFICATES.insert(domain.to_lowercase(), config.clone());
        }
        // compare
        if certificate.updated_at > last_sync {
            last_sync = certificate.updated_at;
        }
    }
    *LAST_SYNC.write().await = last_sync;
    Ok(())
}

fn lookup_certificate(host: &str) -> Option<Arc<CertifiedKey>> {
    // if cached
    if let Some(certificate) = CACHE_CERTIFICATES.read().unwrap().get(host) {
        return Some(certificate.clone());
    }
    // if not cached
    if let Some(certificate) = CERTIFICATES.get(host) {
        CACHE_CERTIFICATES.write().unwrap().insert(host.to_string(), certificate.clone(), **CACHE_CERTIFICATES_EXPIRE);
        return Some(certificate.clone());
    }
    // else match for it
    for (domain, certificate) in CERTIFICATES.clone() {
        // try replace * to regexp and match it
        if domain == "*" {
            CACHE_CERTIFICATES.write().unwrap().insert(host.to_string(), certificate.clone(), **CACHE_CERTIFICATES_EXPIRE);
            return Some(certificate.clone());
        }
        // else use regexp and replace * to \w+
        
        let domain_pattern = domain.replace(".", "\\.").replace("*", r"[-\w]+");
        let regexp = Regex::new(&format!("^{domain_pattern}$")).unwrap();
        if regexp.is_match(host) {
            CACHE_CERTIFICATES.write().unwrap().insert(host.to_string(), certificate.clone(), **CACHE_CERTIFICATES_EXPIRE);
            return Some(certificate.clone());
        }
    }
    None
}