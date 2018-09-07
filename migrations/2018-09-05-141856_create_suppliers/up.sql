-- Your SQL goes here

CREATE TABLE suppliers (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NULL,
    last_name VARCHAR NULL,
    company_name VARCHAR NOT NULL,
    email VARCHAR NULL,
    phone VARCHAR NULL
);

CREATE TABLE product_costs (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id),
    cost_id INTEGER NOT NULL REFERENCES costs(id),
    supplier_id INTEGER NOT NULL REFERENCES suppliers(id),
    cost INTEGER NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX product_costs_product_costs_ids ON product_costs (product_id, cost_id);