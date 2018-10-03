-- Your SQL goes here

CREATE TABLE sale_products(
    id SERIAL PRIMARY KEY,
    sale_id INTEGER NOT NULL REFERENCES sales(id),
    product_id INTEGER NOT NULL REFERENCES products(id),
    tax INTEGER,
    amount FLOAT,
    price INTEGER,
    discount INTEGER,
    subtotal FLOAT,
    sub_total_without_discount FLOAT,
    discount_calculated FLOAT,
    taxes_calculated FLOAT,
    total FLOAT,
    observation TEXT
);