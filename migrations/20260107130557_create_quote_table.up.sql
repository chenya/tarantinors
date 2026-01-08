-- Add up migration script here
CREATE TABLE IF NOT EXISTS quote (
    id SERIAL PRIMARY KEY,
    text VARCHAR(128) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
