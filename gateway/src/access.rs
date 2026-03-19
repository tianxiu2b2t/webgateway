use std::
    sync::{Arc, LazyLock, RwLock}
;

use chrono::{DateTime, TimeDelta, Timelike, Utc};
use dashmap::DashMap;
use http_body::SizeHint;
use hyper::{HeaderMap, Uri, Version};
use shared::{
    database::{access::DatabaseAccessLogsModifyRepository, get_database},
    models::access::{AccessCreateRequest, AccessCreateResponse, AccessUpdateRequestSize, AccessUpdateResponseSize, AccessVersion},
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

#[derive(Debug, Clone)]
pub struct ResponseLog {
    pub inner: AccessCreateResponse,
}
impl ResponseLog {
    pub fn new(
        id: ObjectId,
        http_version: Version,
        headers: &HeaderMap,
        status: u16,
        body_length: SizeHint,
        backend_responsed_at: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            inner: AccessCreateResponse {
                id,
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
                backend_responsed_at,
            },
        })
    }
}

static ACCESS_REQUEST_LOGS: LazyLock<DashMap<Arc<DateTime<Utc>>, Vec<AccessCreateRequest>>> =
    LazyLock::new(DashMap::new);
static ACCESS_RESPONSE_LOGS: LazyLock<DashMap<Arc<DateTime<Utc>>, Vec<AccessCreateResponse>>> =
    LazyLock::new(DashMap::new);
static ACCESS_REQUEST_SIZE_LOGS: LazyLock<DashMap<ObjectId, usize>> = LazyLock::new(DashMap::new);
static ACCESS_RESPONSE_SIZE_LOGS: LazyLock<DashMap<ObjectId, usize>> = LazyLock::new(DashMap::new);
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
        match sync_access_request_logs().await {
            Ok(_) => {}
            Err(e) => {
                event!(Level::ERROR, "Failed to sync access request logs: {}", e);
            }
        }
        match sync_access_response_logs().await {
            Ok(_) => {}
            Err(e) => {
                event!(Level::ERROR, "Failed to sync access response logs: {}", e);
            }
        }
        match sync_request_size_logs().await {
            Ok(_) => {}
            Err(e) => {
                event!(Level::ERROR, "Failed to sync access request size logs: {}", e);
            }
        }
        match sync_response_size_logs().await {
            Ok(_) => {}
            Err(e) => {
                event!(Level::ERROR, "Failed to sync access response size logs: {}", e);
            }
        }
        let updated_time = get_database().get_database_time().unwrap();
        let next_time = (TimeDelta::seconds(1) - (updated_time - *last_time))
            .max(TimeDelta::microseconds(100))
            .to_std()?;

        if next_time.is_zero() {
            event!(
                Level::WARN,
                "Access logs update interval is too short, skipping..."
            );
            continue;
        }

        let _ = tokio::time::sleep(next_time).await;
    }
}

fn update_time() {
    let current_time = get_database()
        .get_database_time()
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
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
    ACCESS_RESPONSE_LOGS.retain(|k, _| k > &current_time);
    get_database().insert_batch_access_responses(logs).await?;
    Ok(())
}

async fn sync_request_size_logs() -> anyhow::Result<()> {
    // clone and delete
    let logs = ACCESS_REQUEST_SIZE_LOGS.clone();
    // ACCESS_REQUEST_SIZE_LOGS.retain(|k, _| logs.contains_key(k));
    if logs.is_empty() {
        return Ok(());
    }
    ACCESS_REQUEST_SIZE_LOGS.retain(|k, _| !logs.contains_key(k));
    get_database()
        .update_batch_access_request_size_logs(logs.iter().map(|value| {
            AccessUpdateRequestSize::from((*value.key(), *value.value()))
        }).collect::<Vec<_>>())
        .await?;

    Ok(())
}

async fn sync_response_size_logs() -> anyhow::Result<()> {
    // clone and delete
    let logs = ACCESS_RESPONSE_SIZE_LOGS.clone();
    // ACCESS_REQUEST_SIZE_LOGS.retain(|k, _| logs.contains_key(k));
    if logs.is_empty() {
        return Ok(());
    }
    ACCESS_RESPONSE_SIZE_LOGS.retain(|k, _| !logs.contains_key(k));
    get_database()
        .update_batch_access_response_size_logs(logs.iter().map(|value| {
            AccessUpdateResponseSize::from((*value.key(), *value.value()))
        }).collect::<Vec<_>>())
        .await?;

    Ok(())
}

pub fn add_request_log(log: &RequestLog) {
    let current_time = { CURRENT_TIME.read().unwrap().clone() };
    let mut logs = ACCESS_REQUEST_LOGS.entry(current_time).or_default();
    logs.push(log.inner.clone());
}

pub fn add_response_log(log: &ResponseLog) {
    let current_time = { CURRENT_TIME.read().unwrap().clone() };
    let mut logs = ACCESS_RESPONSE_LOGS.entry(current_time).or_default();
    logs.push(log.inner.clone());
}

pub fn update_request_size_log(id: ObjectId, size: usize) {
    ACCESS_REQUEST_SIZE_LOGS.insert(id, size);
    // .
}

pub fn update_response_size_log(id: ObjectId, size: usize) {
    ACCESS_RESPONSE_SIZE_LOGS.insert(id, size);
    // .
}
