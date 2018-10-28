-- Your SQL goes here

CREATE TYPE sale_status AS ENUM ('draft',
                                 'saved',
                                 'active',
                                 'cancelled',
                                 'payed',
                                 'overdue');
ALTER TABLE sales ADD COLUMN status sale_status NOT NULL DEFAULT 'draft';
