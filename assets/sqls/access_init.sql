CREATE TABLE IF NOT EXISTS access_request_logs (
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
);

CREATE TABLE IF NOT EXISTS access_response_logs (
    id                      TEXT PRIMARY KEY NOT NULL REFERENCES access_request_logs(id),
    status                  UINT2 NOT NULL,
    headers                 JSONB NOT NULL DEFAULT '[]',
    body_length             uint8,
    http_version            TEXT NOT NULL,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    responsed_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    backend_responsed_at    TIMESTAMPTZ DEFAULT NOW(),
    website_id              TEXT
);

CREATE TABLE IF NOT EXISTS access_request_size_logs (
    id                      TEXT PRIMARY KEY NOT NULL,
    request_id              TEXT NOT NULL REFERENCES access_request_logs(id),
    body_length             uint8 NOT NULL,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS access_response_size_logs (
    id                      TEXT PRIMARY KEY NOT NULL,
    response_id             TEXT NOT NULL REFERENCES access_response_logs(id),
    body_length             uint8 NOT NULL,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE INDEX IF NOT EXISTS idx_requested_at ON access_request_logs (requested_at);
CREATE INDEX IF NOT EXISTS idx_responsed_at ON access_response_logs (responsed_at);
CREATE INDEX IF NOT EXISTS idx_access_response_logs_status ON access_response_logs (status);
CREATE INDEX IF NOT EXISTS idx_access_request_logs_website_id ON access_request_logs (website_id);
CREATE INDEX IF NOT EXISTS idx_access_response_logs_website_id ON access_response_logs (website_id);
CREATE INDEX IF NOT EXISTS idx_access_request_size_logs_created_at ON access_request_size_logs (created_at);
CREATE INDEX IF NOT EXISTS idx_access_response_size_logs_created_at ON access_response_size_logs (created_at);
CREATE INDEX IF NOT EXISTS idx_access_request_size_logs_id ON access_request_size_logs (id);
CREATE INDEX IF NOT EXISTS idx_access_response_size_logs_id ON access_response_size_logs (id);
CREATE INDEX IF NOT EXISTS idx_access_request_size_logs_req_id ON access_request_size_logs (request_id);
CREATE INDEX IF NOT EXISTS idx_access_response_size_logs_resp_id ON access_response_size_logs (response_id);

CREATE OR REPLACE VIEW qps_per_second AS
    SELECT
        date_trunc('second', requested_at) AS time,
        COUNT(req.id) AS total_requests,
        COUNT(req.id) AS qps  
    FROM access_request_logs req
    GROUP BY time
    ORDER BY time DESC;

CREATE OR REPLACE VIEW qps_per_5s AS
    SELECT
        to_timestamp(floor(extract(epoch from requested_at) / 5) * 5) AS time,
        COUNT(req.id) AS total_requests,                
        COUNT(req.id) / 5.0 AS avg_qps                   
    FROM access_request_logs req
    GROUP BY time
    ORDER BY time DESC;

CREATE OR REPLACE VIEW daily_traffic_by_website AS
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
