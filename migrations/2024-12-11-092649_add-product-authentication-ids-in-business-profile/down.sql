-- This file should undo anything in `up.sql`
ALTER TABLE business_profile
DROP COLUMN IF EXISTS product_authentication_ids
