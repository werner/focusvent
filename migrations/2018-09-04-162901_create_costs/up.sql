-- Your SQL goes here

CREATE TABLE costs (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE product_costs (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id),
    cost_id INTEGER NOT NULL REFERENCES costs(id),
    cost INTEGER NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX product_costs_product_costs_ids ON product_costs (product_id, cost_id);