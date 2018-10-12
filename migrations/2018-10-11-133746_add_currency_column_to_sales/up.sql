-- Your SQL goes here

ALTER TABLE sales ADD COLUMN currency_id INTEGER NOT NULL REFERENCES currencies(id);