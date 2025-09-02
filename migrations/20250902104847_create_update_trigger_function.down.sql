-- Add down migration script here

DROP TRIGGER IF EXISTS set_timestamp ON person;
DROP TRIGGER IF EXISTS set_timestamp ON genre;
DROP TRIGGER IF EXISTS set_timestamp ON movie;
DROP TRIGGER IF EXISTS set_timestamp ON movie_role;
DROP TRIGGER IF EXISTS set_timestamp ON movie_genre;
DROP TRIGGER IF EXISTS set_timestamp ON award;
DROP TRIGGER IF EXISTS set_timestamp ON award_category;
DROP TRIGGER IF EXISTS set_timestamp ON movie_award;
DROP TRIGGER IF EXISTS set_timestamp ON movie_nomination;
DROP FUNCTION IF EXISTS trigger_set_timestamp;