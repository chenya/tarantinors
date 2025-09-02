-- Add up migration script here
CREATE TABLE IF NOT EXISTS movie_nomination (
    movie_id INT REFERENCES movie(id) ON DELETE CASCADE,
    award_category_id INT REFERENCES award_category(id) ON DELETE CASCADE,
    year INT NOT NULL,
    nominee VARCHAR(128),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (movie_id, award_category_id, year, nominee)
);