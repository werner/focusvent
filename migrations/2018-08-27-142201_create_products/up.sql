-- Your SQL goes here

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NULL,
    stock DOUBLE PRECISION NOT NULL DEFAULT 0.0
);

CREATE UNIQUE INDEX products_name ON products (name);