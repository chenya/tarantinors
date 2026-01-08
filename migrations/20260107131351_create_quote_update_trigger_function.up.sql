-- Add up migration script here

DROP TRIGGER IF EXISTS set_timestamp ON person;
CREATE TRIGGER  set_timestamp
    BEFORE UPDATE ON quote
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp() ;
