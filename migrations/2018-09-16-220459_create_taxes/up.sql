-- Your SQL goes here

CREATE TABLE taxes(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    percentage INTEGER NOT NULL
);
