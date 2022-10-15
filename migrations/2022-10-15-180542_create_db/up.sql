-- Your SQL goes here
CREATE TABLE datasets (
    id SERIAL PRIMARY KEY,
    in_use BOOLEAN NOT NULL,
    data BYTEA NOT NULL,
    created_on TIMESTAMP NOT NULL
);