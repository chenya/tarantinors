-- Indexes for role table

CREATE INDEX idx_role_name ON role(name);

-- Indexes for person table

CREATE INDEX idx_person_name ON person(name);

-- Indexes for genre table

CREATE INDEX idx_genre_name ON genre(name);

-- Indexes for movie table

CREATE INDEX idx_movie_title ON movie(title);


CREATE INDEX idx_movie_release_year ON movie(release_year);


CREATE INDEX idx_movie_rating ON movie(rating);


CREATE INDEX idx_movie_release_date ON movie(release_date);

-- Indexes for movie_role table

CREATE INDEX idx_movie_role_movie_id ON movie_role(movie_id);


CREATE INDEX idx_movie_role_person_id ON movie_role(person_id);


CREATE INDEX idx_movie_role_role_id ON movie_role(role_id);


CREATE INDEX idx_movie_role_person_role ON movie_role(person_id, role_id);

-- Indexes for movie_genre table

CREATE INDEX idx_movie_genre_movie_id ON movie_genre(movie_id);


CREATE INDEX idx_movie_genre_genre_id ON movie_genre(genre_id);

-- Indexes for award table

CREATE INDEX idx_award_name ON award(name);

-- Indexes for award_category table

CREATE INDEX idx_award_category_award_id ON award_category(award_id);

-- Indexes for movie_award table

CREATE INDEX idx_movie_award_movie_id ON movie_award(movie_id);


CREATE INDEX idx_movie_award_award_category_id ON movie_award(award_category_id);


CREATE INDEX idx_movie_award_year ON movie_award(year);

-- Indexes for movie_nomination table

CREATE INDEX idx_movie_nomination_movie_id ON movie_nomination(movie_id);


CREATE INDEX idx_movie_nomination_award_category_id ON movie_nomination(award_category_id);


CREATE INDEX idx_movie_nomination_year ON movie_nomination(year);