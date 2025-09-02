-- Add up migration script here
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_timestamp ON person;
CREATE TRIGGER  set_timestamp
    BEFORE UPDATE ON person
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp() ;


DROP TRIGGER IF EXISTS set_timestamp ON genre;
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON genre
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


DROP TRIGGER IF EXISTS set_timestamp ON movie;
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON movie
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


DROP TRIGGER IF EXISTS set_timestamp ON movie_role;
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON movie_role
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


DROP TRIGGER IF EXISTS set_timestamp ON movie_genre;
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON movie_genre
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


DROP TRIGGER IF EXISTS set_timestamp ON award;
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON award
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


DROP TRIGGER IF EXISTS set_timestamp ON award_category;
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON award_category
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


DROP TRIGGER IF EXISTS set_timestamp ON movie_award;
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON movie_award
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

DROP TRIGGER IF EXISTS set_timestamp ON movie_nomination;
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON movie_nomination
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();