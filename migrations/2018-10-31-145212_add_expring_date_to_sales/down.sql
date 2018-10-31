-- This file should undo anything in `up.sql`

ALTER TABLE sales DROP COLUMN expiring_date RESTRICT;
