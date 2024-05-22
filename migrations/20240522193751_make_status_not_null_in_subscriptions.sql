-- Add migration script here
BEGIN;

-- backfill status to be 'confirmed'
UPDATE subscriptions
SET status = 'confirmed'
WHERE status IS NULL;

-- status is now mandatory
ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;

COMMIT;
