-- Your SQL goes here

CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE product_prices (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id),
    price_id INTEGER NOT NULL REFERENCES prices(id),
    price INTEGER NOT NULL DEFAULT 0
);
