use reqwest::header::HeaderName;
use ring::{digest, hmac};
use serde_json::{Value, json};
use std::str::FromStr;

use crate::{
    base::{BaseDNS, BaseRecord},
    error::{DNSProviderError, DNSProviderResult},
};
use utils::bytes_to_hex;

const URL: &str = "dnspod.tencentcloudapi.com";
const TENCENT_HEADERS: &[&str] = &["content-type", "host"];
const DATE: &str = "2021-03-23";

#[derive(Debug, Clone)]
pub struct DNSPod {
    key_id: String,
    secret: String,
    user_agent: String,
}

#[derive(Debug, Clone)]
pub struct DNSPodRecord {
    pub id: u64,
    pub domain: String,
    pub hostname: String,
    pub record_type: String,
    pub record_value: String,
}
impl DNSPodRecord {
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl BaseRecord for DNSPodRecord {
    fn domain(&self) -> &str {
        self.domain.as_str()
    }
    fn hostname(&self) -> &str {
        self.hostname.as_str()
    }
    fn record_type(&self) -> &str {
        self.record_type.as_str()
    }
    fn record_value(&self) -> &str {
        self.record_value.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct DNSPodResponse {
    value: Value,
}

impl DNSPodResponse {
    pub fn success(&self) -> bool {
        // contains "Error" in value
        self.value.get("Error").is_none()
    }
    pub fn get_error_message(&self) -> Option<String> {
        match self.value.get("Error") {
            Some(error) => {
                let request_id = self.value.get("RequestId").unwrap().as_str().unwrap();
                let error_code = error.get("Code").unwrap().as_str().unwrap();
                let message = error.get("Message").unwrap().as_str().unwrap();
                Some(format!(
                    "Request id: {request_id}, error code: {error_code}, message: {message}"
                ))
            }
            None => None,
        }
    }

    pub fn data(&self) -> &Value {
        &self.value
    }
}

pub trait DNSPodResponseExt {
    #[allow(clippy::wrong_self_convention)]
    fn as_dnspod_response(self) -> impl Future<Output = DNSProviderResult<DNSPodResponse>>;
}

impl DNSPodResponseExt for reqwest::Response {
    async fn as_dnspod_response(self) -> DNSProviderResult<DNSPodResponse> {
        let value = self
            .json::<Value>()
            .await
            .map_err(DNSProviderError::ResponseJsonDecodeError)?;
        let resp = DNSPodResponse {
            value: value.as_object().unwrap().get("Response").unwrap().clone(),
        };
        if !resp.success() {
            return Err(DNSProviderError::ResponseError(
                resp.get_error_message().unwrap_or_default(),
            ));
        }
        Ok(resp)
    }
}

macro_rules! signature {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        let mut key = $first.as_bytes().to_vec();
        $(
            let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, &key);
            key = hmac::sign(&hmac_key, $rest.as_bytes()).as_ref().to_vec();
        )*
        key
    }};
}

impl DNSPod {
    pub fn new(key_id: &str, secret: &str) -> Self {
        Self::new_with_user_agent(
            key_id,
            secret,
            &format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        )
    }

    pub fn new_with_user_agent(key_id: &str, secret: &str, user_agent: &str) -> Self {
        Self {
            key_id: key_id.to_string(),
            secret: secret.to_string(),
            user_agent: user_agent.to_string(),
        }
    }

    async fn post_action(
        &self,
        action: &str,
        data: Option<&Value>,
    ) -> DNSProviderResult<DNSPodResponse> {
        // Prepare request content
        let content = serde_json::to_string(data.unwrap_or(&json!({}))).unwrap();
        let datetime = chrono::Utc::now();
        let timestamp = datetime.timestamp();
        let timestamp_str = timestamp.to_string();

        // Prepare headers
        let raw_headers: Vec<(&str, &str)> = vec![
            ("Content-Type", "application/json"),
            ("Host", URL),
            ("User-Agent", self.user_agent.as_str()),
            ("X-TC-Action", action),
            ("X-TC-Client", self.user_agent.as_str()),
            ("X-TC-Timestamp", timestamp_str.as_str()),
            ("X-TC-Version", DATE),
        ];

        // Canonical headers and keys for signature
        let headers_str = raw_headers
            .iter()
            .filter(|(k, _)| TENCENT_HEADERS.contains(&k.to_ascii_lowercase().as_str()))
            .map(|(k, v)| format!("{}:{}", k.to_ascii_lowercase(), v.to_ascii_lowercase()))
            .collect::<Vec<_>>()
            .join("\n");

        let headers_keys = TENCENT_HEADERS.join(";");

        let date = datetime.format("%Y-%m-%d").to_string();

        // Step 1: 计算签名密钥
        let sign_key = signature!(format!("TC3{}", self.secret), date, "dnspod", "tc3_request");

        // Step 2: 构造待签名字符串
        let content_sha256 =
            bytes_to_hex(digest::digest(&digest::SHA256, content.as_bytes()).as_ref());
        let canonical_req = format!("POST\n/\n\n{headers_str}\n\n{headers_keys}\n{content_sha256}");
        let canonical_req_hash =
            bytes_to_hex(digest::digest(&digest::SHA256, canonical_req.as_bytes()).as_ref());

        let string_to_sign = format!(
            "TC3-HMAC-SHA256\n{timestamp}\n{date}/dnspod/tc3_request\n{canonical_req_hash}"
        );

        // Step 3: 计算最终签名
        let key = hmac::Key::new(hmac::HMAC_SHA256, &sign_key);
        let sign = bytes_to_hex(hmac::sign(&key, string_to_sign.as_bytes()).as_ref());

        // Step 4: 构造 Authorization
        let authorization = format!(
            "TC3-HMAC-SHA256 Credential={}/{date}/dnspod/tc3_request, SignedHeaders={headers_keys}, Signature={sign}",
            self.key_id
        );

        // Step 5: 发起请求
        let client = reqwest::ClientBuilder::new().build().unwrap();
        let req = client
            .post(format!("https://{URL}/"))
            .header("Authorization", authorization)
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                for (k, v) in raw_headers {
                    let k = HeaderName::from_str(k).unwrap();
                    headers.insert(k, v.parse().unwrap());
                }
                headers
            })
            .body(content);

        let response = req.send().await.map_err(DNSProviderError::RequestError)?;
        response.as_dnspod_response().await
    }
}

impl BaseDNS<DNSPodRecord> for DNSPod {
    async fn list_records(&self, domain: &str) -> DNSProviderResult<Vec<DNSPodRecord>> {
        let resp = self
            .post_action(
                "DescribeRecordList",
                Some(&json!({
                    "Domain": domain,
                })),
            )
            .await?;
        let mut res = Vec::new();
        for record in resp.data().as_object().unwrap()["RecordList"]
            .as_array()
            .unwrap()
        {
            res.push(DNSPodRecord {
                id: record["RecordId"].as_u64().unwrap(),
                domain: domain.to_string(),
                hostname: record["Name"].as_str().unwrap().to_string(),
                record_type: record["Type"].as_str().unwrap().to_string(),
                record_value: record["Value"].as_str().unwrap().to_string(),
            });
        }
        Ok(res)
    }

    async fn add_record(
        &self,
        domain: &str,
        hostname: &str,
        record_type: &str,
        record_value: &str,
    ) -> DNSProviderResult<()> {
        let _resp = self
            .post_action(
                "CreateRecord",
                Some(&json!({
                    "Domain": domain,
                    "SubDomain": hostname,
                    "RecordType": record_type,
                    "RecordLine": "默认",
                    "Value": record_value,
                })),
            )
            .await?;
        Ok(())
    }

    async fn remove_record(
        &self,
        domain: &str,
        hostname: &str,
        record_type: &str,
    ) -> DNSProviderResult<Option<String>> {
        // 先查，再删
        let records = self.list_records(domain).await?;
        for record in records {
            if record.hostname == hostname && record.record_type == record_type {
                let _ = self
                    .post_action(
                        "DeleteRecord",
                        Some(&json!({
                            "Domain": domain,
                            "RecordId": record.id
                        })),
                    )
                    .await?;
                return Ok(Some(record.record_value));
            }
        }
        Ok(None)
    }

    async fn modify_record(
        &self,
        domain: &str,
        hostname: &str,
        record_type: &str,
        record_value: &str,
    ) -> DNSProviderResult<Option<String>> {
        let records = self.list_records(domain).await?;
        for record in records {
            if record.hostname == hostname && record.record_type == record_type {
                if record.record_value == record_value {
                    return Ok(Some(record.record_value));
                }
                let _ = self
                    .post_action(
                        "ModifyRecord",
                        Some(&json!({
                            "Domain": domain,
                            "SubDomain": hostname,
                            "RecordId": record.id,
                            "RecordType": record_type,
                            "RecordLine": "默认",
                            "Value": record_value,
                        })),
                    )
                    .await?;
                return Ok(Some(record.record_value));
            }
        }
        Ok(None)
    }

    async fn get_record(
        &self,
        domain: &str,
        hostname: &str,
        record_type: &str,
    ) -> DNSProviderResult<Option<String>> {
        let records = self.list_records(domain).await?;
        for record in records {
            if record.hostname == hostname && record.record_type == record_type {
                return Ok(Some(record.record_value.clone()));
            }
        }
        Ok(None)
    }
}
