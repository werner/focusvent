-- Your SQL goes here

CREATE TABLE sale_products(
    id SERIAL PRIMARY KEY,
    sale_id INTEGER NOT NULL REFERENCES sales(id),
    product_id INTEGER NOT NULL REFERENCES products(id),
    tax INTEGER NOT NULL,
    amount FLOAT NOT NULL,
    price INTEGER NOT NULL,
    discount INTEGER NOT NULL,
    subtotal INTEGER NOT NULL,
    sub_total_without_discount INTEGER NOT NULL,
    discount_calculated INTEGER NOT NULL,
    taxes_calculated INTEGER NOT NULL,
    total INTEGER NOT NULL,
    observation TEXT
);
