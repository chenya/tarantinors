-- Add up migration script here
CREATE TABLE IF NOT EXISTS interview (
    id SERIAL PRIMARY KEY,
    title VARCHAR(128) NOT NULL UNIQUE,
    youtube_id VARCHAR(20) NOT NULL UNIQUE,
    description VARCHAR(200) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
