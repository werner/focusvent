-- Your SQL goes here

CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NULL,
    last_name VARCHAR NULL,
    company_name VARCHAR NULL,
    email VARCHAR NULL,
    phone VARCHAR NULL
);

CREATE UNIQUE INDEX clients_names_idx ON clients (first_name, last_name, company_name);