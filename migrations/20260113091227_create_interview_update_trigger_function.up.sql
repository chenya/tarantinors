-- Add up migration script here
DROP TRIGGER IF EXISTS set_timestamp ON interview;
CREATE TRIGGER  set_timestamp
    BEFORE UPDATE ON interview
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp() ;
