use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use acmex::{
    AcmeConfig, ChallengeSolverRegistry, Contact, Dns01Solver, DnsProvider,
};
use shared::{
    database::{
        certificate::DatabaseCertificateRepository, dnsprovider::DatabaseDNSProviderQuery,
        get_database,
    },
    models::certificate::{NeedSignCertificate, UpdateCertificate},
    objectid::ObjectId,
};
use tokio::{sync::RwLock, task::JoinHandle};
use tokio_schedule::Job;
use tracing::{Level, event};

pub static PENDINGS: LazyLock<RwLock<HashMap<ObjectId, JoinHandle<()>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn init() -> anyhow::Result<()> {
    tokio::spawn(tokio_schedule::every(30).seconds().perform(|| async {
        if let Err(e) = fetch_new_certificates().await {
            event!(Level::ERROR, "Failed to sync config: {e}");
        }
    }));
    Ok(())
}

pub async fn get_expired() -> anyhow::Result<Vec<NeedSignCertificate>> {
    let certificates = get_database().get_will_sign_certificates().await?;

    let mut res = vec![];
    let pendings = PENDINGS.read().await;
    for certificate in certificates {
        if !pendings.contains_key(&certificate.id) {
            res.push(certificate);
        }
    }
    Ok(res)
}

pub async fn fetch_new_certificates() -> anyhow::Result<()> {
    let expired = get_expired().await?;
    for certificate in expired {
        // TODO: check if certificate is already in PENDINGS
        let id = certificate.id;
        event!(Level::INFO, "Start to sign certificate: {id}");
        let handle = tokio::spawn(async move {
            sign(certificate).await;
        });
        PENDINGS.write().await.insert(id, handle);
    }
    Ok(())
}

async fn inner_sign(cert: NeedSignCertificate) -> anyhow::Result<()> {
    let dns = get_database()
        .get_dns_provider_by_id(&cert.dns_provider_id)
        .await?;
    let mut client = acmex::AcmeClient::new(
        AcmeConfig::new("https://acme.zerossl.com/v2/DV90")
            .with_contact(Contact::email(cert.email))
            .with_tos_agreed(true),
    )?;
    let dns_provider: Arc<dyn DnsProvider> = Arc::new(match dns.provider {
        shared::models::dnsprovider::DatabaseDNSProviderKind::TENCENT(tencent) => {
            acmex::dns::providers::TencentCloudDnsProvider::new(
                tencent.secret_id,
                tencent.secret_key,
                "".to_string(),
            )
        }
        // _ => unreachable!(),
    });

    let mut solver_registry = ChallengeSolverRegistry::new();
    for domain in dns.domains {
        solver_registry.register(Dns01Solver::new(dns_provider.clone(), domain));
    }

    let bundle = client
        .issue_certificate(cert.hostnames, &mut solver_registry)
        .await?;
    let fullchain = bundle.certificate_pem;
    let key = bundle.private_key_pem;
    let final_cert = UpdateCertificate::new(cert.id, fullchain, key);
    get_database().update_certificate(&final_cert).await?;

    Ok(())
}

pub async fn sign(cert: NeedSignCertificate) {
    let id = cert.id;
    if let Err(e) = inner_sign(cert).await {
        event!(Level::ERROR, "Failed to sign [{id}] certificate: {e}");
    } else {
        event!(Level::INFO, "Finish to sign certificate: {id}");
    }
    // remove task from PENDINGS
    let mut pendings = PENDINGS.write().await;
    pendings.remove(&id);
}
