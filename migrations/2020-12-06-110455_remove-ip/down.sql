-- This file should undo anything in `up.sql`
ALTER TABLE questions ADD COLUMN ip_address text not null default '';
