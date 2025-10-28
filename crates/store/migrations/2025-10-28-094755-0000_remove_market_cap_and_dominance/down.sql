-- This file should undo anything in `up.sql`
ALTER TABLE market_data
ADD COLUMN market_cap_usd DOUBLE PRECISION,
ADD COLUMN dominance DOUBLE PRECISION;