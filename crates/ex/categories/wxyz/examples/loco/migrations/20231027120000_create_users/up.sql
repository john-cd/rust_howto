-- migrations/20231027120000_create_users/up.sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL
);


