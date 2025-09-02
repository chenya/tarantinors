-- Add up migration script here
CREATE TABLE IF NOT EXISTS movie_genre (
    movie_id INT REFERENCES movie(id) ON DELETE CASCADE,
    genre_id INT REFERENCES genre(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (movie_id, genre_id)
);
