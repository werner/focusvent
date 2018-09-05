-- Your SQL goes here

CREATE TABLE costs (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE INDEX costs_name_idx ON costs (name varchar_pattern_ops);