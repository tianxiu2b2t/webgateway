use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::objectid::ObjectId;
use sqlx::{FromRow, Row, postgres::PgRow, types::Json};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub content: LogContent,
    pub created_at: DateTime<Utc>,
    pub address: String,
}

impl<'r> FromRow<'r, PgRow> for Log {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            content: row.try_get::<Json<LogContent>, _>("content")?.0,
            created_at: row.try_get("created_at")?,
            address: row.try_get("address")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum LogContent {
    Raw(String),
    Data(LogContentData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContentParams {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContentData {
    pub content: String,
    pub params: Vec<LogContentParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogQueryParams {
    pub limit: Option<usize>,
    pub page: Option<usize>,
}
