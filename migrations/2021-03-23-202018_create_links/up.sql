-- Your SQL goes here

CREATE TABLE links (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    url VARCHAR NOT NULL,
    public BOOLEAN NOT NULL DEFAULT false
)