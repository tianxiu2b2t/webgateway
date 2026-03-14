use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use simple_shared::objectid::ObjectId;
use sqlx::{
    FromRow, Row,
    postgres::PgRow,
    types::{Json, Text},
};
use sqlx_pg_ext_uint::{c_u16::U16, c_usize::USize};

#[derive(Debug, Clone)]
pub enum AccessVersion {
    HTTP09,
    HTTP10,
    HTTP11,
    HTTP2,
    HTTP3,
}

impl<'de> Deserialize<'de> for AccessVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "http/1.1" => Ok(AccessVersion::HTTP11),
            "http/1.0" => Ok(AccessVersion::HTTP10),
            "http/2.0" => Ok(AccessVersion::HTTP2),
            "http/0.9" => Ok(AccessVersion::HTTP09),
            "http/3.0" => Ok(AccessVersion::HTTP3),
            _ => Err(serde::de::Error::custom("Invalid HTTP version")),
        }
    }
}

impl Serialize for AccessVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AccessVersion::HTTP10 => serializer.serialize_str("HTTP/1.0"),
            AccessVersion::HTTP11 => serializer.serialize_str("HTTP/1.1"),
            AccessVersion::HTTP2 => serializer.serialize_str("HTTP/2.0"),
            AccessVersion::HTTP3 => serializer.serialize_str("HTTP/3.0"),
            AccessVersion::HTTP09 => serializer.serialize_str("HTTP/0.9"),
        }
    }
}

impl FromStr for AccessVersion {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "http/1.1" => Ok(AccessVersion::HTTP11),
            "http/1.0" => Ok(AccessVersion::HTTP10),
            "http/2.0" => Ok(AccessVersion::HTTP2),
            "http/0.9" => Ok(AccessVersion::HTTP09),
            "http/3.0" => Ok(AccessVersion::HTTP3),
            _ => Err(std::fmt::Error),
        }
    }
}

// to str
impl std::fmt::Display for AccessVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessVersion::HTTP10 => write!(f, "HTTP/1.0"),
            AccessVersion::HTTP11 => write!(f, "HTTP/1.1"),
            AccessVersion::HTTP2 => write!(f, "HTTP/2.0"),
            AccessVersion::HTTP3 => write!(f, "HTTP/3.0"),
            AccessVersion::HTTP09 => write!(f, "HTTP/0.9"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRequest {
    pub id: ObjectId,
    pub host: String,
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub http_version: AccessVersion,
    pub remote_addr: String,
    pub body_length: usize,
    pub created_at: DateTime<Utc>,
    pub requested_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for AccessRequest {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            method: row.try_get("method")?,
            path: row.try_get("path")?,
            headers: row.try_get::<Json<Vec<_>>, _>("headers")?.0,
            http_version: row.try_get::<Text<AccessVersion>, _>("http_version")?.0,
            remote_addr: row.try_get("remote_addr")?,
            created_at: row.try_get("created_at")?,
            requested_at: row.try_get("requested_at")?,
            body_length: row.try_get::<USize, _>("body_length")?.into(),
            host: row.try_get("host")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCreateRequest {
    pub id: ObjectId,
    pub host: String,
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub http_version: AccessVersion,
    pub remote_addr: String,
    pub body_length: usize,
    pub requested_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessResponse {
    pub id: ObjectId,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body_length: Option<usize>,
    pub http_version: AccessVersion,
    pub created_at: DateTime<Utc>,
    pub responsed_at: DateTime<Utc>,
    pub backend_request_at: DateTime<Utc>,
    pub backend_response_at: Option<DateTime<Utc>>,
}

impl<'r> FromRow<'r, PgRow> for AccessResponse {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            status: row.try_get::<U16, _>("status")?.into(),
            headers: row.try_get::<Json<Vec<_>>, _>("headers")?.0,
            created_at: row.try_get("created_at")?,
            responsed_at: row.try_get("responsed_at")?,
            body_length: row
                .try_get::<Option<USize>, _>("body_length")?
                .map(|v| v.into()),
            http_version: row.try_get::<Text<AccessVersion>, _>("http_version")?.0,
            backend_request_at: row.try_get("backend_request_at")?,
            backend_response_at: row.try_get("backend_response_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCreateResponse {
    pub id: ObjectId,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body_length: usize,
    pub http_version: AccessVersion,
    pub responsed_at: DateTime<Utc>,
    pub backend_request_at: DateTime<Utc>,
    pub backend_response_at: Option<DateTime<Utc>>,
}
