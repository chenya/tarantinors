-- Add up migration script here
CREATE TABLE IF NOT EXISTS  award_category (
    id SERIAL PRIMARY KEY,
    award_id INT REFERENCES award(id) ON DELETE CASCADE,
    category VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (award_id, category)
);
