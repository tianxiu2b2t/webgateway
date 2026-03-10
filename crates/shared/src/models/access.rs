use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use simple_shared::objectid::ObjectId;
use sqlx::{FromRow, Row, postgres::PgRow, types::Json};
use sqlx_pg_ext_uint::c_u16::U16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRequest {
    pub id: ObjectId,
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub http_version: String,
    pub remote_addr: String,
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
            http_version: row.try_get("http_version")?,
            remote_addr: row.try_get("remote_addr")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
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

