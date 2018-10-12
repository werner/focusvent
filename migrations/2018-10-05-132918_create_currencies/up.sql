-- Your SQL goes here

CREATE TABLE currencies(
    id SERIAL PRIMARY KEY,
    value VARCHAR NOT NULL,
    symbol VARCHAR NOT NULL,
    decimal_point VARCHAR NOT NULL,
    default_currency BOOLEAN NOT NULL DEFAULT false,
    in_use BOOLEAN NOT NULL DEFAULT false
);
