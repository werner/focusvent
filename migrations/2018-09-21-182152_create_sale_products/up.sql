-- Your SQL goes here

CREATE TABLE sale_products(
    id SERIAL PRIMARY KEY,
    sale_id INTEGER NOT NULL REFERENCES sales(id),
    tax_id INTEGER NOT NULL REFERENCES taxes(id),
    product_id INTEGER NOT NULL REFERENCES products(id),
    amount FLOAT,
    price INTEGER,
    discount INTEGER,
    subtotal INTEGER
);