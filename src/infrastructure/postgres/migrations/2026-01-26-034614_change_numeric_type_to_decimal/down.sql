-- This file should undo anything in `up.sql`
ALTER TABLE zones
ALTER COLUMN price TYPE NUMERIC(10, 2);

ALTER TABLE bookings
ALTER COLUMN total_price TYPE NUMERIC(10, 2);