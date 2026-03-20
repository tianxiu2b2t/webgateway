use std::sync::LazyLock;

use crate::{
    database::Database,
    models::access::{
        AccessCreateRequest, AccessCreateResponse, AccessInfo, AccessInsertRequestSize,
        AccessInsertResponseSize, AccessUpdateRequestSize, AccessUpdateResponseSize, DatabaseQPS,
        ResponseQPS,
    },
};
use async_trait::async_trait;
use sqlx::{QueryBuilder, types::Json};
use sqlx_pg_ext_uint::{c_u16::U16, c_usize::USize};

static CREATE_TABLE_SQLS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        r#"CREATE TABLE IF NOT EXISTS access_request_logs (
            id                      TEXT PRIMARY KEY NOT NULL,
            host                    TEXT NOT NULL,
            method                  TEXT NOT NULL,
            path                    TEXT NOT NULL,
            headers                 JSONB NOT NULL DEFAULT '[]',
            http_version            TEXT NOT NULL,
            remote_addr             TEXT NOT NULL,
            body_length             uint8 NOT NULL,
            created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            requested_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            website_id              TEXT
        )
        "#,
        r#"CREATE TABLE IF NOT EXISTS access_response_logs (
            id                      TEXT PRIMARY KEY NOT NULL REFERENCES access_request_logs(id),
            status                  UINT2 NOT NULL,
            headers                 JSONB NOT NULL DEFAULT '[]',
            body_length             uint8,
            http_version            TEXT NOT NULL,
            created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            responsed_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            backend_responsed_at    TIMESTAMPTZ DEFAULT NOW(),
            website_id              TEXT
        )
        "#,
        r#"CREATE TABLE IF NOT EXISTS access_request_size_logs(
            id                      TEXT PRIMARY KEY NOT NULL REFERENCES access_request_logs(id),
            body_length             uint8 NOT NULL,
            created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        )"#,
        r#"CREATE TABLE IF NOT EXISTS access_response_size_logs(
            id                      TEXT PRIMARY KEY NOT NULL REFERENCES access_response_logs(id),
            body_length             uint8 NOT NULL,
            created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
    ]
});

static CREATE_INDEX_SQLS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        "CREATE INDEX IF NOT EXISTS idx_requested_at ON access_request_logs (requested_at);",
        "CREATE INDEX IF NOT EXISTS idx_responsed_at ON access_response_logs (responsed_at);",
        // "CREATE INDEX IF NOT EXISTS idx_access_request_logs_host_requested_at ON access_request_logs (host, requested_at);",
        "CREATE INDEX IF NOT EXISTS idx_access_response_logs_status ON access_response_logs (status);",
        "CREATE INDEX IF NOT EXISTS idx_access_request_logs_website_id ON access_request_logs (website_id);",
        "CREATE INDEX IF NOT EXISTS idx_access_response_logs_website_id ON access_response_logs (website_id);",
        "CREATE INDEX IF NOT EXISTS idx_access_request_size_logs_created_at ON access_request_size_logs (created_at);",
        "CREATE INDEX IF NOT EXISTS idx_access_response_size_logs_created_at ON access_response_size_logs (created_at);",
        "CREATE INDEX IF NOT EXISTS idx_access_request_size_logs_id ON access_request_size_logs (id);",
        "CREATE INDEX IF NOT EXISTS idx_access_response_size_logs_id ON access_response_size_logs (id);",
    ]
});

static CREATE_VIEW_SQLS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        r#"CREATE OR REPLACE VIEW qps_per_second AS
            SELECT
                date_trunc('second', requested_at) AS time,
                COUNT(req.id) AS total_requests,
                COUNT(req.id) AS qps  
            FROM access_request_logs req
            GROUP BY time
            ORDER BY time DESC;
        "#,
        // 替换原有的 qps_per_5s 视图
        r#"CREATE OR REPLACE VIEW qps_per_5s AS
            SELECT
                to_timestamp(floor(extract(epoch from requested_at) / 5) * 5) AS time,
                COUNT(req.id) AS total_requests,                
                COUNT(req.id) / 5.0 AS avg_qps                   
            FROM access_request_logs req
            GROUP BY time
            ORDER BY time DESC;"
        #,
        r#"CREATE OR REPLACE VIEW daily_traffic_by_website AS
        SELECT
            DATE(req.requested_at) AS day,
            COALESCE(req.website_id, 'global') AS website_id,
            COUNT(req.id) AS total_requests,
            COUNT(resp.id) AS total_responses,
            COALESCE(SUM(req.body_length), 0) AS total_request_bytes,
            COALESCE(SUM(resp.body_length), 0) AS total_response_bytes,
            COALESCE(SUM(req.body_length), 0) + COALESCE(SUM(resp.body_length), 0) AS total_bytes
        FROM access_request_logs req
        LEFT JOIN access_response_logs resp ON req.id = resp.id
        GROUP BY day, req.website_id;
        "#,
    ]
});

#[async_trait]
pub trait DatabaseAccessLogsInitializer {
    async fn initialize_access_logs(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl DatabaseAccessLogsInitializer for Database {
    async fn initialize_access_logs(&self) -> anyhow::Result<()> {
        let tables = CREATE_TABLE_SQLS.clone();
        let indexes = CREATE_INDEX_SQLS.clone();
        let views = CREATE_VIEW_SQLS.clone();
        for sql in [tables, indexes, views]
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>()
        {
            sqlx::query(sql).execute(&self.pool).await?;
        }

        Ok(())
    }
}

// 以下为占位的空实现，可根据后续需求填充方法
#[async_trait]
pub trait DatabaseAccessLogsRepository {
    async fn get_qps_per_second(&self, count: usize) -> anyhow::Result<ResponseQPS>;
    async fn get_qps_per_5s(&self, count: usize) -> anyhow::Result<ResponseQPS>;
    async fn get_access_info(&self, in_days: usize) -> anyhow::Result<AccessInfo>;
}

#[async_trait]
impl DatabaseAccessLogsRepository for Database {
    async fn get_qps_per_second(&self, count: usize) -> anyhow::Result<ResponseQPS> {
        let max_limit = count;
        let rows = sqlx::query_as::<_, DatabaseQPS>(
            "SELECT time, total_requests, qps FROM qps_per_secondWHERE time >= NOW() - INTERVAL '1 second' * $1 ORDER BY time DESC LIMIT $1",
        )
        .bind(max_limit as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(ResponseQPS {
            interval: 1,
            data: rows,
            current_time: self.get_database_time()?,
        })
    }
    async fn get_qps_per_5s(&self, count: usize) -> anyhow::Result<ResponseQPS> {
        let max_limit = count * 5;
        let rows = sqlx::query_as::<_, DatabaseQPS>
            ("SELECT time, total_requests FROM qps_per_5s WHERE time >= NOW() - INTERVAL '1 second' * $1 ORDER BY time DESC LIMIT $1")
            .bind(max_limit as i64)
            .fetch_all(&self.pool)
            .await?;
        Ok(ResponseQPS {
            interval: 5,
            data: rows,
            current_time: self.get_database_time()?,
        })
    }
    async fn get_access_info(&self, in_days: usize) -> anyhow::Result<AccessInfo> {
        // 使用 LEFT JOIN 关联请求表和响应表，一次性获取所有统计指标
        let row = sqlx::query_as::<_, (i64, i64, i64, i64, i64)>(
            r#"
            SELECT
                COUNT(req.id) AS total_requests,
                COUNT(DISTINCT req.remote_addr) AS total_ips,
                COUNT(resp.id) FILTER (WHERE resp.status >= 400 AND resp.status <= 499) AS e4xx_requests,
                COUNT(resp.id) FILTER (WHERE resp.status >= 500 AND resp.status <= 599) AS e5xx_requests,
                COUNT(req.id) FILTER (WHERE resp.id IS NULL) AS backend_error_requests
            FROM access_request_logs req
            LEFT JOIN access_response_logs resp ON req.id = resp.id
            WHERE req.requested_at > NOW() - INTERVAL '1 day' * $1
            "#,
        )
        .bind(in_days as i64)  // 绑定天数参数
        .fetch_one(&self.pool)
        .await?;

        // 将数据库返回的 i64 转换为 usize（注意溢出风险，通常天数范围内的请求数不会超过 usize 最大值）
        Ok(AccessInfo {
            total_requests: row.0 as usize,
            total_ips: row.1 as usize,
            e4xx_requests: row.2 as usize,
            e5xx_requests: row.3 as usize,
            backend_error_requests: row.4 as usize,
        })
    }
}

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
    async fn update_batch_access_request_size_logs(
        &self,
        requests: Vec<AccessUpdateRequestSize>,
    ) -> anyhow::Result<()>;
    async fn update_batch_access_response_size_logs(
        &self,
        responses: Vec<AccessUpdateResponseSize>,
    ) -> anyhow::Result<()>;
    async fn insert_batch_access_response_increase_size_logs(
        &self,
        responses: Vec<AccessInsertResponseSize>,
    ) -> anyhow::Result<()>;
    async fn insert_batch_access_request_increase_size_logs(
        &self,
        requests: Vec<AccessInsertRequestSize>,
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
            "INSERT INTO access_request_logs (id, host, method, path, headers, http_version, remote_addr, body_length, requested_at, website_id)",
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
                .push_bind(req.requested_at)
                .push_bind(req.website_id);
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
            "INSERT INTO access_response_logs (id, status, headers, body_length, http_version, backend_responsed_at, responsed_at, website_id)",
        );
        builder.push_values(responses.iter(), |mut b, resp| {
            b.push_bind(resp.id)
                .push_bind(U16::from(resp.status))
                .push_bind(Json(&resp.headers))
                .push_bind(USize::from(resp.body_length))
                .push_bind(resp.http_version.to_string())
                .push_bind(resp.backend_responsed_at)
                .push_bind(resp.responsed_at)
                .push_bind(resp.website_id);
        });
        builder.build().execute(&self.pool).await?;
        Ok(())
    }

    async fn update_batch_access_request_size_logs(
        &self,
        requests: Vec<AccessUpdateRequestSize>,
    ) -> anyhow::Result<()> {
        if requests.is_empty() {
            return Ok(());
        }

        // 构建 UPDATE 语句，使用 CASE 表达式一次更新多行
        let mut builder =
            sqlx::QueryBuilder::new("UPDATE access_request_logs SET body_length = CASE");

        for req in &requests {
            // 追加 WHEN id = ? THEN ? 子句
            builder.push(" WHEN id = ");
            builder.push_bind(req.id); // id 是 String (ObjectId 转文本)
            builder.push(" THEN ");
            builder.push_bind(USize::from(req.body_length)); // body_length: usize -> uint8
        }
        builder.push(" ELSE body_length END "); // 其余行保持原值

        // 添加 WHERE 子句限定要更新的行，避免全表扫描
        builder.push(" WHERE id IN (");
        let mut separated = builder.separated(", ");
        for req in &requests {
            separated.push_bind(req.id); // 再次绑定 id 用于 IN 列表
        }
        builder.push(")");

        // 执行批量更新
        builder.build().execute(&self.pool).await?;

        Ok(())
    }
    async fn update_batch_access_response_size_logs(
        &self,
        responses: Vec<AccessUpdateResponseSize>,
    ) -> anyhow::Result<()> {
        if responses.is_empty() {
            return Ok(());
        }

        // 构建 UPDATE 语句，使用 CASE 表达式一次更新多行
        let mut builder =
            sqlx::QueryBuilder::new("UPDATE access_response_logs SET body_length = CASE");

        for req in &responses {
            // 追加 WHEN id = ? THEN ? 子句
            builder.push(" WHEN id = ");
            builder.push_bind(req.id); // id 是 String (ObjectId 转文本)
            builder.push(" THEN ");
            builder.push_bind(USize::from(req.body_length)); // body_length: usize -> uint8
        }
        builder.push(" ELSE body_length END "); // 其余行保持原值

        // 添加 WHERE 子句限定要更新的行，避免全表扫描
        builder.push(" WHERE id IN (");
        let mut separated = builder.separated(", ");
        for req in &responses {
            separated.push_bind(req.id); // 再次绑定 id 用于 IN 列表
        }
        builder.push(")");

        // 执行批量更新
        builder.build().execute(&self.pool).await?;

        Ok(())
    }

    async fn insert_batch_access_response_increase_size_logs(
        &self,
        responses: Vec<AccessInsertResponseSize>,
    ) -> anyhow::Result<()> {
        if responses.is_empty() {
            return Ok(());
        }
        let mut builder = QueryBuilder::new(
            "INSERT INTO access_response_logs (id, body_length, created_at) VALUES",
        );
        builder.push_values(responses.iter(), |mut b, resp| {
            b.push_bind(resp.id)
                .push_bind(USize::from(resp.body_length))
                .push_bind(resp.created_at);
        });
        builder.build().execute(&self.pool).await?;
        Ok(())
    }

    async fn insert_batch_access_request_increase_size_logs(
        &self,
        requests: Vec<AccessInsertRequestSize>,
    ) -> anyhow::Result<()> {
        if requests.is_empty() {
            return Ok(());
        }
        let mut builder = QueryBuilder::new(
            "INSERT INTO access_request_logs (id, body_length, created_at) VALUES",
        );
        builder.push_values(requests.iter(), |mut b, req| {
            b.push_bind(req.id)
                .push_bind(USize::from(req.body_length))
                .push_bind(req.created_at);
        });
        builder.build().execute(&self.pool).await?;
        Ok(())
    }
}
