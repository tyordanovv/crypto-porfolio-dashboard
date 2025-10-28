-- Your SQL goes here
ALTER TABLE market_data
DROP COLUMN IF EXISTS market_cap_usd,
DROP COLUMN IF EXISTS dominance;