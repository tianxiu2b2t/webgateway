use crate::{
    database::Database,
    models::access::{AccessCreateRequest, AccessCreateResponse},
};
use async_trait::async_trait;
use sqlx::{QueryBuilder, types::Json};
use sqlx_pg_ext_uint::{c_u16::U16, c_usize::USize};

#[async_trait]
pub trait DatabaseAccessLogsInitializer {
    async fn initialize_access_logs(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl DatabaseAccessLogsInitializer for Database {
    async fn initialize_access_logs(&self) -> anyhow::Result<()> {
        for sql in [
            r#"CREATE TABLE IF NOT EXISTS access_request_logs (
                id             TEXT PRIMARY KEY NOT NULL,
                host           TEXT NOT NULL,
                method         TEXT NOT NULL,
                path           TEXT NOT NULL,
                headers        JSONB NOT NULL DEFAULT '[]',
                http_version   TEXT NOT NULL,
                remote_addr    TEXT NOT NULL,
                body_length    uint8 NOT NULL,
                created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                requested_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
            r#"CREATE TABLE IF NOT EXISTS access_response_logs (
                id                  TEXT PRIMARY KEY NOT NULL REFERENCES access_request_logs(id),
                status              UINT2 NOT NULL,
                headers             JSONB NOT NULL DEFAULT '[]',
                body_length         uint8,
                http_version        TEXT NOT NULL,
                created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                responsed_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                backend_request_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                backend_response_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
            "CREATE INDEX IF NOT EXISTS idx_requested_at ON access_request_logs (requested_at);",
            r#"CREATE OR REPLACE VIEW qps_per_second AS
                SELECT
                    date_trunc('second', requested_at) AS second,
                    COUNT(*) AS total_requests,
                    COUNT(*) AS qps  
                FROM access_request_logs
                GROUP BY second
                ORDER BY second DESC;
            "#,
            r#"CREATE OR REPLACE VIEW qps_per_5s AS
                SELECT
                    to_timestamp(floor(extract(epoch from requested_at) / 5) * 5) AS window_start,
                    COUNT(*) AS total_requests,                
                    COUNT(*) / 5.0 AS avg_qps                   
                FROM access_request_logs
                GROUP BY window_start
                ORDER BY window_start DESC;
            "#,
            // 加速根据 host 统计 qps
            "CREATE INDEX IF NOT EXISTS idx_access_request_logs_host_requested_at ON access_request_logs (host, requested_at);",
            "CREATE INDEX IF NOT EXISTS idx_access_response_logs_status ON access_response_logs (status);",
            // 视图
            r#"
            CREATE OR REPLACE VIEW qps_per_second_by_host_status AS
            SELECT 
                date_trunc('second', req.requested_at) AS second,
                req.host,
                resp.status,
                COUNT(*) AS requests_per_second
            FROM access_request_logs req
            JOIN access_response_logs resp ON req.id = resp.id
            GROUP BY second, req.host, resp.status
            ORDER BY second DESC, req.host, resp.status;
            "#,
            r#"
            CREATE OR REPLACE VIEW qps_per_5s_by_host_status AS
            SELECT 
                to_timestamp(floor(extract(epoch from req.requested_at) / 5) * 5) AS window_start,
                req.host,
                resp.status,
                COUNT(*) AS total_requests,
                COUNT(*) / 5.0 AS avg_qps
            FROM access_request_logs req
            JOIN access_response_logs resp ON req.id = resp.id
            GROUP BY window_start, req.host, resp.status
            ORDER BY window_start DESC, req.host, resp.status;
            "#,
        ] {
            sqlx::query(sql).execute(&self.pool).await?;
        }

        Ok(())
    }
}

// 以下为占位的空实现，可根据后续需求填充方法
#[async_trait]
pub trait DatabaseAccessLogsRepository {}

#[async_trait]
impl DatabaseAccessLogsRepository for Database {}

#[async_trait]
pub trait DatabaseAccessLogsModifyRepository {
    async fn insert_batch_access_requests(
        &self,
        requests: Vec<AccessCreateRequest>,
    ) -> anyhow::Result<()>;
    async fn insert_batch_access_responses(
        &self,
        responses: Vec<AccessCreateResponse>,
    ) -> anyhow::Result<()>;
}

#[async_trait]
impl DatabaseAccessLogsModifyRepository for Database {
    async fn insert_batch_access_requests(
        &self,
        requests: Vec<AccessCreateRequest>,
    ) -> anyhow::Result<()> {
        if requests.is_empty() {
            return Ok(());
        }
        let mut builder = QueryBuilder::new(
            "INSERT INTO access_request_logs (id, host, method, path, headers, http_version, remote_addr, body_length, requested_at)",
        );
        builder.push_values(requests.iter(), |mut b, req| {
            b.push_bind(req.id)
                .push_bind(&req.host)
                .push_bind(&req.method)
                .push_bind(&req.path)
                .push_bind(Json(&req.headers))
                .push_bind(req.http_version.to_string())
                .push_bind(&req.remote_addr)
                .push_bind(USize::from(req.body_length))
                .push_bind(req.requested_at);
        });
        builder.build().execute(&self.pool).await?;
        Ok(())
    }
    async fn insert_batch_access_responses(
        &self,
        responses: Vec<AccessCreateResponse>,
    ) -> anyhow::Result<()> {
        if responses.is_empty() {
            return Ok(());
        }
        let mut builder = QueryBuilder::new(
            "INSERT INTO access_response_logs (id, status, headers, body_length, http_version, backend_request_at, backend_response_at, responsed_at)",
        );
        builder.push_values(responses.iter(), |mut b, resp| {
            b.push_bind(resp.id)
                .push_bind(U16::from(resp.status))
                .push_bind(Json(&resp.headers))
                .push_bind(USize::from(resp.body_length))
                .push_bind(resp.http_version.to_string())
                .push_bind(resp.backend_request_at)
                .push_bind(resp.backend_response_at)
                .push_bind(resp.responsed_at);
        });
        builder.build().execute(&self.pool).await?;
        Ok(())
    }
}
