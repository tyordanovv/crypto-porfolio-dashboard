use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicator {
    pub name: String,
    pub category: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentData {
    pub name: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategySignal {
    pub id: Uuid,
    pub asset_symbol: String,
    pub timestamp: chrono::NaiveDateTime,
    pub signal_type: String,
    pub value: Option<f64>,
    pub description: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub asset_symbol: String,
    pub timestamp: NaiveDate,
    pub price_usd: f64,
    pub volume_usd: Option<f64>,
    pub market_cap_usd: Option<f64>,
    pub dominance: Option<f64>,
}