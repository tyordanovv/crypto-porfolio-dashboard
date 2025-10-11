-- Your SQL goes here
-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Market data: one row per asset per day
CREATE TABLE IF NOT EXISTS market_data (
    asset_symbol VARCHAR(16) NOT NULL,
    timestamp DATE NOT NULL,
    price_usd DOUBLE PRECISION NOT NULL,
    volume_usd DOUBLE PRECISION,
    market_cap_usd DOUBLE PRECISION,
    dominance DOUBLE PRECISION,
    PRIMARY KEY (asset_symbol, timestamp)
);

-- Indicators: one row per indicator per day
CREATE TABLE IF NOT EXISTS indicators (
    name VARCHAR(128) NOT NULL,
    category VARCHAR(64) NOT NULL,
    timestamp DATE NOT NULL,
    value DOUBLE PRECISION,
    source VARCHAR(64),
    PRIMARY KEY (name, timestamp)
);

-- Sentiment data: one row per sentiment type per day
CREATE TABLE IF NOT EXISTS sentiment_data (
    name VARCHAR(128) NOT NULL,
    timestamp DATE NOT NULL,
    value DOUBLE PRECISION,
    source VARCHAR(64),
    PRIMARY KEY (name, timestamp)
);

-- Strategy signals: still use UUIDs (each strategy run can produce many signals)
CREATE TABLE IF NOT EXISTS strategy_signals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    asset_symbol VARCHAR(16) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    signal_type VARCHAR(64) NOT NULL,
    value DOUBLE PRECISION,
    description TEXT,
    source VARCHAR(64),
    UNIQUE (asset_symbol, signal_type, timestamp)
);
