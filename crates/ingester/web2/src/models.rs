use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FearGreedIndex {
    pub value: f64,
    pub classification: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FredIndicator {
    pub series_id: String,
    pub value: f64,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPrice {
    pub symbol: String,
    pub price_usd: f64,
    pub market_cap_usd: Option<f64>,
    pub volume_24h_usd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMarketData {
    pub total_market_cap_usd: f64,
    pub total_stable_cap_usd: f64,
    pub total_volume_24h_usd: f64,
}

// API Response structures (internal)
#[derive(Debug, Deserialize)]
pub(crate) struct FearGreedResponse {
    pub data: Vec<FearGreedItem>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FearGreedItem {
    pub value: String,
    pub value_classification: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FredResponse {
    pub observations: Vec<FredObservation>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FredObservation {
    pub date: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct CoinGeckoSimplePrice {
    pub usd: f64,
    pub usd_market_cap: Option<f64>,
    pub usd_24h_vol: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinGeckoGlobal {
    pub data: CoinGeckoGlobalData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinGeckoGlobalData {
    pub total_market_cap: std::collections::HashMap<String, f64>,
    pub total_volume: std::collections::HashMap<String, f64>,
    pub market_cap_percentage: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinMarketCapResponseData {
    pub data: CoinMarketCapData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinMarketCapData {
    pub points: Vec<CoinMarketCapPoint>,
    pub yearlyPerformance: CoinMarketCapYearlyPerformance,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinMarketCapPoint {
    pub marketCap: f64,
    pub stableValue: f64,
    pub volume: f64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinMarketCapYearlyPerformance {
    pub high: CoinMarketCapPerfPoint,
    pub low: CoinMarketCapPerfPoint,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinMarketCapPerfPoint {
    pub marketCap: f64,
    pub timestamp: String,
}