use chrono::NaiveDate;
use uuid::Uuid;
use diesel::{prelude::{Identifiable, Insertable, Queryable}};
use serde::{Deserialize, Serialize};
use crate::schema::{market_data, indicators, sentiment_data, strategy_signals};

//
// MarketData
//
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct MarketData {
    pub asset_symbol: String,
    pub timestamp: NaiveDate,
    pub price_usd: f64,
    pub volume_usd: Option<f64>,
    pub market_cap_usd: Option<f64>,
    pub dominance: Option<f64>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = market_data)]
pub struct NewMarketData {
    pub asset_symbol: String,
    pub timestamp: NaiveDate,
    pub price_usd: f64,
    pub volume_usd: Option<f64>,
    pub market_cap_usd: Option<f64>,
    pub dominance: Option<f64>,
}

//
// Indicator
//
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(primary_key(name, timestamp))]
pub struct Indicator {
    pub name: String,
    pub category: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = indicators)]
pub struct NewIndicator {
    pub name: String,
    pub category: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}

//
// SentimentData
//
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sentiment_data)]
#[diesel(primary_key(name, timestamp))]
pub struct SentimentData {
    pub name: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sentiment_data)]
pub struct NewSentimentData {
    pub name: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}

//
// StrategySignal
//
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(primary_key(id))]
pub struct StrategySignal {
    pub id: Uuid,
    pub asset_symbol: String,
    pub timestamp: chrono::NaiveDateTime,
    pub signal_type: String,
    pub value: Option<f64>,
    pub description: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = strategy_signals)]
pub struct NewStrategySignal {
    pub asset_symbol: String,
    pub signal_type: String,
    pub value: Option<f64>,
    pub description: Option<String>,
    pub source: Option<String>,
}