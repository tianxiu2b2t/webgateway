// use std::time::Duration;

// use crate::default::default_website_config_timeout;
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use url::Url;
// use utils::default_false;

pub mod certificate;
pub mod dns;
pub mod websites;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum ProxyProtocolType {
//     V1,
//     V2,
// }

// /// 负载均衡url
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct LoadBalancerUrl {
//     balance: usize,
//     url: Url,
//     source_host_type: SourceHostType,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum SourceHostType {
//     /// 三个，一个是穿透
//     Transport,
//     Custom(Option<String>),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct WebSite {
//     id: String,
//     /// 需要匹配的域名
//     hosts: Vec<String>,
//     /// 源站，也就是回源流量
//     source: Vec<LoadBalancerUrl>,
//     config: WebSiteConfig,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct WebSiteConfig {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     proxy_protocol: Option<ProxyProtocolType>,
//     #[serde(default = "default_false")]
//     http2: bool,
//     #[serde(default = "default_false")]
//     grpc: bool,
//     #[serde(default = "default_false")]
//     websocket: bool,
//     #[serde(default = "default_website_config_timeout")]
//     timeout: Duration,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct WebSiteCertificates {
//     hostnames: Vec<String>,
//     fullchain: String,
//     private_key: String,
//     created_at: DateTime<Utc>,
//     updated_at: DateTime<Utc>,
//     expired_at: DateTime<Utc>,
// }
