use std::sync::{Arc, LazyLock, RwLock};

use chrono::{DateTime, TimeDelta, Timelike, Utc};
use dashmap::DashMap;
use http_body::SizeHint;
use hyper::{HeaderMap, Uri, Version};
use shared::{
    database::{access::DatabaseAccessLogsModifyRepository, get_database},
    models::access::{AccessCreateRequest, AccessCreateResponse, AccessVersion},
    objectid::ObjectId,
};
use tracing::{Level, event};

#[derive(Debug, Clone)]
pub struct RequestLog {
    pub inner: AccessCreateRequest,
}

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub req_id: ObjectId,
    pub host: String,
    pub uri: Uri,
    pub headers: HeaderMap,
    pub method: hyper::Method,
    pub version: Version,
    pub body_length: SizeHint,
    pub remote_addr: String,
}

impl RequestLog {
    pub fn new(context: RequestContext) -> anyhow::Result<Self> {
        Ok(Self {
            inner: AccessCreateRequest {
                id: context.req_id,
                method: context.method.to_string(),
                path: match context.uri.path_and_query() {
                    Some(v) => v.to_string(),
                    None => "".to_string(),
                },
                headers: {
                    let mut converted_headers = vec![];
                    for (k, v) in context.headers.iter() {
                        converted_headers
                            .push((k.to_string(), v.to_str().unwrap_or_default().to_string()));
                    }
                    converted_headers
                },
                host: context.host,
                http_version: {
                    match context.version {
                        Version::HTTP_09 => AccessVersion::HTTP09,
                        Version::HTTP_10 => AccessVersion::HTTP10,
                        Version::HTTP_11 => AccessVersion::HTTP11,
                        Version::HTTP_2 => AccessVersion::HTTP2,
                        Version::HTTP_3 => AccessVersion::HTTP3,
                        version => return Err(anyhow::anyhow!("Unknown version: {version:?}")),
                    }
                },
                remote_addr: context.remote_addr,
                body_length: context.body_length.lower().try_into().unwrap(),
                requested_at: get_database().get_database_time().unwrap(),
            },
        })
    }
}
/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCreateResponse {
    pub id: ObjectId,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body_length: usize,
    pub responsed_at: DateTime<Utc>,
    pub backend_request_at: DateTime<Utc>,
    pub backend_response_at: Option<DateTime<Utc>>,
} */

#[derive(Debug, Clone)]
pub struct ResponseLog {
    pub inner: AccessCreateResponse,
}
impl ResponseLog {
    pub fn new(
        context: RequestContext,
        http_version: Version,
        headers: &HeaderMap,
        status: u16,
        body_length: SizeHint,
        backend_request_at: DateTime<Utc>,
        backend_response_at: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            inner: AccessCreateResponse {
                id: context.req_id,
                status,
                headers: {
                    let mut converted_headers = vec![];
                    for (k, v) in headers.iter() {
                        converted_headers
                            .push((k.to_string(), v.to_str().unwrap_or_default().to_string()));
                    }
                    converted_headers
                },
                http_version: {
                    match http_version {
                        Version::HTTP_09 => AccessVersion::HTTP09,
                        Version::HTTP_10 => AccessVersion::HTTP10,
                        Version::HTTP_11 => AccessVersion::HTTP11,
                        Version::HTTP_2 => AccessVersion::HTTP2,
                        Version::HTTP_3 => AccessVersion::HTTP3,
                        version => return Err(anyhow::anyhow!("Unknown version: {version:?}")),
                    }
                },
                body_length: body_length.lower().try_into().unwrap(),
                responsed_at: get_database().get_database_time().unwrap(),
                backend_request_at,
                backend_response_at,
            },
        })
    }
}

static ACCESS_REQUEST_LOGS: LazyLock<DashMap<Arc<DateTime<Utc>>, Vec<AccessCreateRequest>>> =
    LazyLock::new(DashMap::new);
static ACCESS_RESPONSE_LOGS: LazyLock<DashMap<Arc<DateTime<Utc>>, Vec<AccessCreateResponse>>> =
    LazyLock::new(DashMap::new);
static CURRENT_TIME: LazyLock<RwLock<Arc<DateTime<Utc>>>> =
    LazyLock::new(|| RwLock::new(Arc::new(Utc::now())));
pub async fn init_access_logs() -> anyhow::Result<()> {
    tokio::spawn(async move {
        let r = background_update_access_logs().await;
        if let Err(e) = r {
            event!(Level::ERROR, "Failed to update access logs: {}", e);
        }
    });
    Ok(())
}

pub async fn background_update_access_logs() -> anyhow::Result<()> {
    let time = get_database().get_real_database_time().await?;
    let next_time = (time + TimeDelta::seconds(1)).with_nanosecond(0).unwrap();
    // util next 1s
    let offset = (next_time - time).to_std()?;
    let _ = tokio::time::sleep(offset).await;
    loop {
        let last_time = { CURRENT_TIME.read().unwrap().clone() };
        update_time();
        let res = match tokio::try_join!(
            tokio::spawn(sync_access_request_logs()),
            tokio::spawn(sync_access_response_logs())
        ) {
            Ok((res1, res2)) => {
                if let Err(e) = res1 {
                    event!(Level::ERROR, "Failed to sync access logs: {}", e);
                }
                if let Err(e) = res2 {
                    event!(Level::ERROR, "Failed to sync access logs: {}", e);
                }
                Ok(())
            }
            Err(e) => Err(e),
        };
        if let Err(e) = res {
            event!(Level::ERROR, "Failed to sync access logs: {}", e);
        }
        let updated_time = Utc::now();
        let next_time = (TimeDelta::seconds(1) - (updated_time - *last_time))
            .max(TimeDelta::seconds(0))
            .to_std()?;

        let _ = tokio::time::sleep(next_time).await;
    }
}

fn update_time() {
    let current_time = Utc::now().with_nanosecond(0).unwrap();
    let mut time = CURRENT_TIME.write().unwrap();
    *time = Arc::new(current_time);
}

async fn sync_access_request_logs() -> anyhow::Result<()> {
    let logs = ACCESS_REQUEST_LOGS.clone();
    let current_time = { CURRENT_TIME.read().unwrap().clone() };
    // fetch before current_time
    let logs = logs
        .iter()
        .filter_map(|v| match v.key() < &current_time {
            true => Some(v.value().clone()),
            false => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    if logs.is_empty() {
        return Ok(());
    }
    // first clean old
    ACCESS_REQUEST_LOGS.retain(|k, _| k > &current_time);
    get_database().insert_batch_access_requests(logs).await?;
    Ok(())
}

async fn sync_access_response_logs() -> anyhow::Result<()> {
    let logs = ACCESS_RESPONSE_LOGS.clone();
    let current_time = { CURRENT_TIME.read().unwrap().clone() };
    // fetch before current_time
    let logs = logs
        .iter()
        .filter_map(|v| match v.key() < &current_time {
            true => Some(v.value().clone()),
            false => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    if logs.is_empty() {
        return Ok(());
    }
    // first clean old
    ACCESS_REQUEST_LOGS.retain(|k, _| k > &current_time);
    get_database().insert_batch_access_responses(logs).await?;
    Ok(())
}

pub fn add_request_log(log: &RequestLog) {
    let current_time = { CURRENT_TIME.read().unwrap().clone() };
    let mut logs = ACCESS_REQUEST_LOGS.entry(current_time).or_default();
    logs.push(log.inner.clone());
}
