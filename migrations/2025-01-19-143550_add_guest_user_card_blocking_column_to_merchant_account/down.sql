-- This file should undo anything in `up.sql`

ALTER TABLE merchant_account
        DROP COLUMN guest_user_card_blocking;