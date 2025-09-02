-- Add up migration script here
CREATE TABLE IF NOT EXISTS award (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
