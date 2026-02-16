use std::{sync::Arc, time::Duration};

use rcgen::{CertifiedKey, generate_simple_self_signed};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
};

pub fn default_website_config_timeout() -> Duration {
    Duration::from_secs(10)
}

pub fn default_dashboard_api_port() -> u16 {
    3000
}

pub fn default_dashboard_database_max_connections() -> u32 {
    10
}

pub fn sign_default_certificates() -> anyhow::Result<Arc<ServerConfig>> {
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();

    // Own the certificate DER data
    let cert_der_bytes = cert.der().to_vec();
    let cert_chain = vec![CertificateDer::from(cert_der_bytes)];

    // Own the private key DER data (PKCS#8)
    let key_der_bytes = signing_key.serialize_der();
    let private_key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key_der_bytes));

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)?;

    Ok(Arc::new(config))
}
