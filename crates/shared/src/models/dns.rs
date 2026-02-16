use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DNSProviderType {
    TENCENT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSProviderTencentConfig {
    pub secret_id: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSProvider {
    pub id: String,
    #[serde(flatten)]
    pub provider: DNSProviderKind,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSProviderUpdate {
    pub id: String,
    #[serde(flatten)]
    pub provider: DNSProviderKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSProviderCreate {
    #[serde(flatten)]
    pub provider: DNSProviderKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum DNSProviderKind {
    Tencent(DNSProviderTencentConfig),
}

impl DNSProviderKind {
    pub fn get_type(&self) -> DNSProviderType {
        match self {
            DNSProviderKind::Tencent(_) => DNSProviderType::TENCENT,
        }
    }
}
