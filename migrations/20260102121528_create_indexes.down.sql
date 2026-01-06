-- Add down migration script here
-- Drop indexes for role table
DROP INDEX IF EXISTS idx_role_name;

-- Drop indexes for person table
DROP INDEX IF EXISTS idx_person_name;

-- Drop indexes for genre table
DROP INDEX IF EXISTS idx_genre_name;

-- Drop indexes for movie table
DROP INDEX IF EXISTS idx_movie_title;
DROP INDEX IF EXISTS idx_movie_release_year;
DROP INDEX IF EXISTS idx_movie_rating;
DROP INDEX IF EXISTS idx_movie_release_date;

-- Drop indexes for movie_role table
DROP INDEX IF EXISTS idx_movie_role_movie_id;
DROP INDEX IF EXISTS idx_movie_role_person_id;
DROP INDEX IF EXISTS idx_movie_role_role_id;
DROP INDEX IF EXISTS idx_movie_role_person_role;

-- Drop indexes for movie_genre table
DROP INDEX IF EXISTS idx_movie_genre_movie_id;
DROP INDEX IF EXISTS idx_movie_genre_genre_id;

-- Drop indexes for award table
DROP INDEX IF EXISTS idx_award_name;

-- Drop indexes for award_category table
DROP INDEX IF EXISTS idx_award_category_award_id;

-- Drop indexes for movie_award table
DROP INDEX IF EXISTS idx_movie_award_movie_id;
DROP INDEX IF EXISTS idx_movie_award_award_category_id;
DROP INDEX IF EXISTS idx_movie_award_year;

-- Drop indexes for movie_nomination table
DROP INDEX IF EXISTS idx_movie_nomination_movie_id;
DROP INDEX IF EXISTS idx_movie_nomination_award_category_id;
DROP INDEX IF EXISTS idx_movie_nomination_year;