-- This file should undo anything in `up.sql`

ALTER TABLE sales DROP COLUMN status RESTRICT;
DROP TYPE sale_status;
