use std::net::{Ipv4Addr, Ipv6Addr};

use anyhow::Context;
use chrono::{DateTime, Utc};
use pem::parse_many;
use rcgen::KeyPair;
use rustls::{ServerConfig, pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject}};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};
use utils::replace_sensitive_data;
use x509_parser::prelude::{FromDer, GeneralName, X509Certificate};

use crate::{objectid::ObjectId, secret::RemovedSensitiveInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCertificate {
    pub id: ObjectId,
    pub name: Option<String>,
    pub hostnames: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub fullchain: String,
    pub private_key: String,
    pub dns_provider_id: Option<ObjectId>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'a> DatabaseCertificate {
    pub fn get_fullchain(&self) -> anyhow::Result<Vec<CertificateDer<'a>>> {
        Ok(parse_many(&self.fullchain)?.iter().map(|v| {
            let content = v.contents();
            CertificateDer::from_slice(content).into_owned()
        }).collect::<Vec<CertificateDer>>())
    }
    pub fn get_private_key(&self) -> anyhow::Result<PrivateKeyDer<'a>> {
        Ok(PrivateKeyDer::from_pem_slice(self.private_key.as_bytes())?)
    }
    pub fn get_server_config(&'a self) -> anyhow::Result<rustls::ServerConfig> {
        let fullchain = self.get_fullchain()?.to_owned();
        let private_key = self.get_private_key()?;
        Ok(ServerConfig::builder().with_no_client_auth().with_single_cert(fullchain, private_key)?)
    }
}

impl RemovedSensitiveInfo for DatabaseCertificate {
    fn remove_sensitive_info(&self) -> Self {
        Self {
            id: self.id,
            hostnames: self.hostnames.clone(),
            expires_at: self.expires_at,
            fullchain: String::new(),
            private_key: String::new(),
            dns_provider_id: self.dns_provider_id,
            email: self.email.as_ref().map(replace_sensitive_data),
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: self.name.clone()
        }
    }
}

impl<'r> FromRow<'r, PgRow> for DatabaseCertificate {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            hostnames: row.try_get("hostnames")?,
            expires_at: row.try_get("expires_at")?,
            fullchain: row.try_get("fullchain")?,
            private_key: row.try_get("private_key")?,
            dns_provider_id: row.try_get("dns_provider_id")?,
            email: row.try_get("email")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            name: row.try_get("name")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeedSignCertificate {
    pub id: ObjectId,
    pub hostnames: Vec<String>,
    pub dns_provider_id: ObjectId,
    pub email: String,
}

impl<'r> FromRow<'r, PgRow> for NeedSignCertificate {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            hostnames: row.try_get("hostnames")?,
            dns_provider_id: row.try_get("dns_provider_id")?,
            email: row.try_get("email")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCertificate {
    pub id: ObjectId,
    pub fullchain: String,
    pub private_key: String,
}

impl UpdateCertificate {
    pub fn new(id: ObjectId, fullchain: String, private_key: String) -> Self {
        Self {
            id,
            fullchain,
            private_key,
        }
    }

    pub fn expires_at(&self) -> anyhow::Result<DateTime<Utc>> {
        get_expired_at(&self.fullchain)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCertificate {
    pub name: Option<String>,
    #[serde(flatten)]
    pub content: CreateCertificateMethod,
}
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum CreateCertificateMethod {
    AUTO(CreateCertificateAuto),
    MANUAL(CreateCertificateManual),
}

impl<'de> Deserialize<'de> for CreateCertificateMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {   
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "type")]
            method: String,
            content: serde_json::Value,
        }

        let helper = Helper::deserialize(deserializer)?;
        match helper.method.to_uppercase().as_str() {
            "AUTO" => Ok(Self::AUTO(CreateCertificateAuto::deserialize(helper.content).map_err(|_| serde::de::Error::custom("Invaild CreateCertificateAuto"))?)),
            "MANUAL" => Ok(Self::MANUAL(CreateCertificateManual::deserialize(helper.content).map_err(|_| serde::de::Error::custom("Invaild CreateCertificateManual"))?)),
            _ => Err(serde::de::Error::custom("Invalid CreateCertificateMethod")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCertificateAuto {
    pub dns_provider_id: ObjectId,
    pub email: String,
    pub hostnames: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCertificateManual {
    pub fullchain: String,
    pub private_key: String,
}

impl CreateCertificateManual {
    // check vaild
    pub fn check(&self) -> anyhow::Result<()> {
        // 查看是不是一对的
        if self.fullchain.is_empty() || self.private_key.is_empty() {
            return Err(anyhow::anyhow!("fullchain or private_key is empty"));
        }

        Ok(())
    }

    pub fn expires_at(&self) -> anyhow::Result<DateTime<Utc>> {
        get_expired_at(&self.fullchain)
    }

    pub fn hostnames(&self) -> anyhow::Result<Vec<String>> {
        get_hostnames(&self.fullchain)
    }

    pub fn vaild(&self) -> anyhow::Result<()> {
        check_match(&self.fullchain, &self.private_key)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateQueryParams {
    pub limit: Option<usize>,
    pub page: Option<usize>,
}

fn get_expired_at(fullchain: &str) -> anyhow::Result<DateTime<Utc>> {
        // 1. Parse the PEM blocks from the fullchain string
        let pem_blocks = parse_many(fullchain.as_bytes())?;
        // 2. Get the first certificate (leaf)
        let leaf_pem = pem_blocks
            .first()
            .ok_or(anyhow::anyhow!("Failed to get first certificate"))?;
        // 3. Parse the DER contents as an X.509 certificate
        let (_, cert) = X509Certificate::from_der(leaf_pem.contents())?;
        // 4. Extract the `not_after` timestamp
        let not_after = cert.validity().not_after;
        // 5. Convert to chrono::DateTime<Utc>
        //    X.509 timestamps can be in UTC (Z) or with offsets; `not_after.timestamp()` gives seconds since epoch.
        Ok(DateTime::from_timestamp(not_after.timestamp(), 0).unwrap())
}

fn get_hostnames(fullchain: &str) -> anyhow::Result<Vec<String>> {
    let pem_blocks = parse_many(fullchain.as_bytes())?;
    let leaf_pem = pem_blocks
        .first()
        .ok_or(anyhow::anyhow!("Failed to get first certificate"))?;
    let (_, cert) = X509Certificate::from_der(leaf_pem.contents())?;
    let mut domains = vec![];
    if let Some(sans) = cert.subject_alternative_name().unwrap_or_default() {
        for san in &sans.value.general_names {
            match san {
                GeneralName::DNSName(dns_name) => {
                    domains.push(dns_name.to_string());
                }
                GeneralName::IPAddress(ip_name) => {
                    if ip_name.len() == 4 {
                        domains.push(Ipv4Addr::from_octets(std::convert::TryInto::<[u8; 4]>::try_into(*ip_name)?).to_string())
                    } else if ip_name.len() == 16 {
                        domains.push(Ipv6Addr::from_octets(std::convert::TryInto::<[u8; 16]>::try_into(*ip_name)?).to_string())
                    } 
                },
                _ => {}
            }
        }
    }
    Ok(domains)
}

fn check_match(fullchain: &str, prikey: &str) -> anyhow::Result<()> {
    // 1. 解析 fullchain，获取叶子证书的裸公钥
    let fullchain_pems = pem::parse_many(fullchain.as_bytes())?;
    let leaf_pem = fullchain_pems
        .iter()
        .find(|p| p.tag() == "CERTIFICATE")
        .ok_or_else(|| anyhow::anyhow!("fullchain 中没有证书"))?;
    let (_, cert) = X509Certificate::from_der(leaf_pem.contents())?;
    // 裸公钥 = BIT STRING 中的实际数据
    let cert_pubkey_raw = cert.public_key().subject_public_key.data.to_vec();

    // 2. 解析私钥，获取裸公钥
    let key_pair = KeyPair::from_pem(prikey)
        .context("无法解析私钥 PEM")?;
    let privkey_pubkey_raw = key_pair.public_key_raw();

    // 3. 比较裸公钥
    println!("cert_pubkey_raw: {:?}", cert_pubkey_raw);
    println!("privkey_pubkey_raw: {:?}", privkey_pubkey_raw);
    if cert_pubkey_raw.as_slice() == privkey_pubkey_raw {
        Ok(())
    } else {
        Err(anyhow::anyhow!("证书与私钥不匹配"))
    }
}