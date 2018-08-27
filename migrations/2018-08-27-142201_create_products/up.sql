-- Your SQL goes here

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NULL,
    stock DOUBLE PRECISION NULL DEFAULT 0.0
);