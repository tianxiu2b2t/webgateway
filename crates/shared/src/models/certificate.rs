// use anyhow::anyhow;
// use chrono::{DateTime, TimeZone, Utc};
// use pem::parse_many;
// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;
// use x509_parser::prelude::{FromDer, GeneralName, X509Certificate};

// use crate::objectid::ObjectId;

// /// 完整的证书数据库记录
// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct DatabaseCertificate {
//     pub id: ObjectId,
//     pub hostnames: Vec<String>, // 从证书解析的 SAN
//     pub fullchain: String,      // 完整的证书链 PEM
//     pub private_key: String,    // 私钥 PEM
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
//     pub dns_provider_id: Option<String>,   // 关联的 DNS 提供商 ID
//     pub expires_at: Option<DateTime<Utc>>, // 从证书解析的过期时间
// }

// /// 创建证书所需的输入（不包含自动计算的字段）
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CreateDatabaseCertificate {
//     pub fullchain: String,
//     pub private_key: String,
//     pub dns_provider_id: Option<String>,
// }

// /// 更新证书的输入（整体替换证书内容）
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UpdateDatabaseCertificate {
//     pub id: ObjectId,
//     pub fullchain: String,
//     pub private_key: String,
//     pub dns_provider_id: Option<String>,
// }

// /// 简化的证书结构，用于返回给客户端
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Certificate {
//     pub id: ObjectId,
//     pub hostnames: Vec<String>,
//     pub expires_at: Option<DateTime<Utc>>,
//     pub dns_provider_id: Option<String>,
// }

// impl From<DatabaseCertificate> for Certificate {
//     fn from(cert: DatabaseCertificate) -> Self {
//         Self {
//             id: cert.id,
//             hostnames: cert.hostnames,
//             expires_at: cert.expires_at,
//             dns_provider_id: cert.dns_provider_id,
//         }
//     }
// }

// pub fn extract_cert_info(pem_data: &str) -> anyhow::Result<(Vec<String>, DateTime<Utc>)> {
//     // 解析 PEM，提取所有证书
//     let pems = parse_many(pem_data.as_bytes())?;

//     // 找到第一个证书（叶子证书）
//     let leaf_pem = pems
//         .iter()
//         .find(|p| p.tag() == "CERTIFICATE")
//         .ok_or_else(|| anyhow!("No Found Any Certificates"))?;

//     // 解析 X.509 证书
//     let (_, cert) = X509Certificate::from_der(leaf_pem.contents())
//         .map_err(|e| anyhow!("Failed to parse X.509 certificate: {}", e))?;

//     // 提取 SAN 中的 DNS 名称
//     let mut hostnames = Vec::new();
//     if let Some(san) = cert.subject_alternative_name()? {
//         for general_name in san.value.general_names.clone() {
//             if let GeneralName::DNSName(name) = general_name {
//                 hostnames.push(name.to_string());
//             }
//         }
//     }

//     // 如果没有 SAN，使用 CN 作为后备
//     if hostnames.is_empty()
//         && let Some(cn) = cert.subject().iter_common_name().next()
//         && let Ok(cn_str) = cn.as_str()
//     {
//         hostnames.push(cn_str.to_string());
//     }

//     if hostnames.is_empty() {
//         return Err(anyhow!("Certificate has no DNS names (SAN or CN)"));
//     }

//     // 提取过期时间
//     let not_after = cert.validity().not_after;
//     let expiry = Utc
//         .timestamp_opt(not_after.timestamp(), 0)
//         .single()
//         .ok_or_else(|| anyhow!("Invalid expiration timestamp"))?;

//     Ok((hostnames, expiry))
// }

// /// 检查证书是否有效（未过期）
// pub fn is_certificate_valid(expires_at: Option<DateTime<Utc>>) -> bool {
//     match expires_at {
//         Some(expiry) => Utc::now() < expiry,
//         None => false,
//     }
// }

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::objectid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub id: ObjectId,
    pub hostnames: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub fullchain: String,
    pub private_key: String,
    pub dns_provider_id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
