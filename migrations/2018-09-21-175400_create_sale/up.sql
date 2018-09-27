-- Your SQL goes here

CREATE TABLE sales(
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id),
    sale_date DATE,
    sub_total INTEGER,
    total INTEGER,
    observation TEXT
);
