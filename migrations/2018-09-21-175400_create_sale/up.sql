-- Your SQL goes here

CREATE TABLE sales(
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id),
    sale_date DATE NOT NULL,
    sub_total FLOAT,
    total FLOAT,
    observation TEXT
);
