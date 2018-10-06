-- Your SQL goes here

CREATE TABLE sales(
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES clients(id),
    sale_date DATE NOT NULL,
    sub_total INTEGER NOT NULL,
    sub_total_without_discount INTEGER NOT NULL,
    discount_calculated INTEGER NOT NULL,
    taxes_calculated INTEGER NOT NULL,
    total INTEGER NOT NULL,
    observation TEXT
);
