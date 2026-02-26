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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow, types::Json};

use crate::objectid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseDNSProvider {
    pub id: ObjectId,
    #[serde(flatten)]
    pub provider: DatabaseDNSProviderKind,
    pub domains: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for DatabaseDNSProvider {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            provider: row
                .try_get::<Json<DatabaseDNSProviderKind>, _>("provider")?
                .0,
            domains: row.try_get("domains")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum DatabaseDNSProviderKind {
    TENCENT(DatabaseDNSProviderTencent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseDNSProviderTencent {
    pub secret_id: String,
    pub secret_key: String,
}
