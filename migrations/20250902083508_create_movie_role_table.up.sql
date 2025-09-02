-- Add up migration script here
CREATE TABLE IF NOT EXISTS movie_role (
    movie_id INT REFERENCES movie(id) ON DELETE CASCADE,
    person_id INT REFERENCES person(id) ON DELETE CASCADE,
    role_id INT REFERENCES role(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (movie_id, person_id, role_id)
);
