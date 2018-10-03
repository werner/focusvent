-- Your SQL goes here

CREATE TABLE sales(
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id),
    sale_date DATE NOT NULL,
    sub_total FLOAT,
    sub_total_without_discount FLOAT,
    discount_calculated FLOAT,
    taxes_calculated FLOAT,
    total FLOAT,
    observation TEXT
);
