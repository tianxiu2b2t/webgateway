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
    HTTP1,
    HTTP11,
    HTTP2,
}

impl<'de> Deserialize<'de> for AccessVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "http/1.1" => Ok(AccessVersion::HTTP11),
            "http/1.0" => Ok(AccessVersion::HTTP1),
            "http/2.0" => Ok(AccessVersion::HTTP2),
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
            AccessVersion::HTTP1 => serializer.serialize_str("HTTP/1.0"),
            AccessVersion::HTTP11 => serializer.serialize_str("HTTP/1.1"),
            AccessVersion::HTTP2 => serializer.serialize_str("HTTP/2.0"),
        }
    }
}

impl FromStr for AccessVersion {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "http/1.1" => Ok(AccessVersion::HTTP11),
            "http/1.0" => Ok(AccessVersion::HTTP1),
            "http/2.0" => Ok(AccessVersion::HTTP2),
            _ => Err(std::fmt::Error),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRequest {
    pub id: ObjectId,
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub http_version: AccessVersion,
    pub remote_addr: String,
    pub body_length: usize,
    pub created_at: String,
    pub updated_at: String,
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
            updated_at: row.try_get("updated_at")?,
            body_length: row.try_get::<USize, _>("body_length")?.into(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCreateRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub http_version: String,
    pub remote_addr: String,
    pub body_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessResponse {
    pub id: ObjectId,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body_length: usize,
    pub http_version: AccessVersion,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for AccessResponse {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            status: row.try_get::<U16, _>("status")?.into(),
            headers: row.try_get::<Json<Vec<_>>, _>("headers")?.0,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            body_length: row.try_get::<USize, _>("body_length")?.into(),
            http_version: row.try_get::<Text<AccessVersion>, _>("http_version")?.0,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCreateResponse {
    pub id: ObjectId,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body_length: usize,
}
