-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    pkey BYTEA NOT NULL
);