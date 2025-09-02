-- Add down migration script here
DELETE FROM role
WHERE name in ('Actor', 'Director', 'Producer', 'Writer');