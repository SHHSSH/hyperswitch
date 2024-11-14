-- Your SQL goes here
CREATE TABLE IF NOT EXISTS call_back_mapper (
    id VARCHAR(128) NOT NULL PRIMARY KEY,
    type VARCHAR(64) NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()::TIMESTAMP,
    last_modified_at TIMESTAMP NOT NULL DEFAULT now()::TIMESTAMP
);