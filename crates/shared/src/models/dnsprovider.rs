// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum DNSProviderType {
//     TENCENT,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DNSProviderTencentConfig {
//     pub secret_id: String,
//     pub secret_key: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DNSProvider {
//     pub id: String,
//     #[serde(flatten)]
//     pub provider: DNSProviderKind,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DNSProviderUpdate {
//     pub id: String,
//     #[serde(flatten)]
//     pub provider: DNSProviderKind,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DNSProviderCreate {
//     #[serde(flatten)]
//     pub provider: DNSProviderKind,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", content = "config")]
// pub enum DNSProviderKind {
//     Tencent(DNSProviderTencentConfig),
// }

// impl DNSProviderKind {
//     pub fn get_type(&self) -> DNSProviderType {
//         match self {
//             DNSProviderKind::Tencent(_) => DNSProviderType::TENCENT,
//         }
//     }
// }

use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Row, postgres::PgRow, types::Json};
use utils::replace_sensitive_data;

use crate::{objectid::ObjectId, secret::RemovedSensitiveInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseDNSProvider {
    pub id: ObjectId,
    #[serde(flatten)]
    pub provider: DatabaseDNSProviderKind,
    pub domains: Vec<String>,
        pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RemovedSensitiveInfo for DatabaseDNSProvider {
    fn remove_sensitive_info(&self) -> Self {
        Self {
            id: self.id,
            provider: self.provider.remove_sensitive_info(),
            domains: self.domains.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: self.name.clone(),
        }
    }
}

impl<'r> FromRow<'r, PgRow> for DatabaseDNSProvider {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            /*
                            provider_type TEXT NOT NULL,
                provider_config JSONB NOT NULL, */
            provider: row
                .try_get::<Json<DatabaseDNSProviderKind>, _>("provider")?
                .0,
            domains: row.try_get("domains")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            name: row.try_get("name")?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum DNSProviderType {
    TENCENT,
}

impl<'de> Deserialize<'de> for DNSProviderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_uppercase();
        Ok(match s.as_str() {
            "TENCENT" => DNSProviderType::TENCENT,
            _ => return Err(serde::de::Error::custom("Invalid DNSProviderType")),
        })
    }
}

impl Display for DNSProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DNSProviderType::TENCENT => write!(f, "TENCENT"),
        }
    }
}

// Encode

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum DatabaseDNSProviderKind {
    TENCENT(DatabaseDNSProviderTencent),
}

impl<'de> Deserialize<'de> for DatabaseDNSProviderKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "type")]
            provider_type: String,
            config: Value,
        }

        let helper = Helper::deserialize(deserializer)?;
        match helper.provider_type.to_uppercase().as_str() {
            "TENCENT" => Ok(Self::TENCENT(DatabaseDNSProviderTencent::deserialize(helper.config).map_err(|_| serde::de::Error::custom("Invaild DNSProviderTencent"))?)),
            _ => Err(serde::de::Error::custom("Invalid DNSProviderType")),
        }
    }
}

impl DatabaseDNSProviderKind {
    pub fn get_type(&self) -> DNSProviderType {
        match self {
            DatabaseDNSProviderKind::TENCENT(_) => DNSProviderType::TENCENT,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseDNSProviderTencent {
    pub secret_id: String,
    pub secret_key: String,
}

impl RemovedSensitiveInfo for DatabaseDNSProviderKind {
    fn remove_sensitive_info(&self) -> Self {
        match self {
            DatabaseDNSProviderKind::TENCENT(provider) => {
                DatabaseDNSProviderKind::TENCENT(provider.remove_sensitive_info())
            }
        }
    }
}

impl RemovedSensitiveInfo for DatabaseDNSProviderTencent {
    fn remove_sensitive_info(&self) -> Self {
        Self {
            secret_id: replace_sensitive_data(&self.secret_id),
            secret_key: replace_sensitive_data(&self.secret_key),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSProviderQueryParams {
    pub limit: Option<usize>,
    pub page: Option<usize>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDatabaseDNSProvider {
    #[serde(flatten)]
    pub provider: DatabaseDNSProviderKind,
    pub domains: Vec<String>,
    pub name: Option<String>,
}