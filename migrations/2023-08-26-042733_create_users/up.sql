-- Ensure the pgcrypto extension is loaded for UUID generation
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create the 'users' table with 'id' as UUID and primary key
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR NOT NULL UNIQUE,
    pkey BYTEA NOT NULL
);