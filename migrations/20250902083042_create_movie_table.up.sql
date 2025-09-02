-- Add up migration script here
CREATE TABLE IF NOT EXISTS movie (
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) UNIQUE NOT NULL,
    release_year INT NOT NULL,
    plot TEXT,
    runtime INT,
    rating REAL,
    release_date DATE,
    image_url TEXT,
    youtube_id VARCHAR(32),
    budget VARCHAR(32),
    production_details TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);