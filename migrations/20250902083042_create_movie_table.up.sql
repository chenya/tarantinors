-- Add up migration script here
CREATE TABLE IF NOT EXISTS movie (
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) UNIQUE NOT NULL,
    release_year INT NOT NULL,
    plot TEXT NOT NULL,
    runtime INT NOT NULL,
    rating REAL NOT NULL,
    release_date DATE NOT NULL,
    image_url TEXT NOT NULL,
    youtube_id VARCHAR(32) NOT NULL,
    budget VARCHAR(32) NOT NULL,
    production_details TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);