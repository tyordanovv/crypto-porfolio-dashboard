use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FearGreedIndex {
    pub value: f64,
    pub classification: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct MarketPrice {
    pub symbol: String,
    pub price_usd: f64,
    pub price_usd_7d_ago: f64,
    pub price_usd_30d_ago: f64,
    pub price_usd_90d_ago: f64,
    pub volume_24h_usd: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FredIndicator {
    pub series_id: String,
    pub value: f64,
    pub date: String,
}

#[derive(Debug)]
pub struct CryptoPrice {
    pub symbol: String,
    pub price_usd: f64,
    pub price_usd_7d_ago: f64,
    pub price_usd_30d_ago: f64,
    pub price_usd_90d_ago: f64,
    pub volume_24h_usd: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooChart {
    pub chart: ChartData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChartData {
    pub result: Option<Vec<ChartResult>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChartResult {
    pub timestamp: Option<Vec<i64>>,
    pub indicators: Indicators,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Indicators {
    pub quote: Vec<Quote>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Quote {
    pub close: Vec<Option<f64>>,
    pub volume: Vec<Option<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCryptoMarketData {
    pub total_market_cap_usd: f64,
    pub total_stable_cap_usd: f64,
    pub total_btc_cap_usd: f64,
    pub total_eth_cap_usd: f64,
    pub total_volume_24h_usd: f64,
}

#[derive(Debug, Deserialize)]
pub struct AdvancedMetrics{
    pub btc_dominance: f64,
    pub eth_dominance: f64,
    pub btc_stable_ratio: f64,
    pub btc_return_7d: f64,
    pub btc_return_30d: f64,
    pub btc_return_90d: f64,
    pub btc_volatility: f64,
    pub btc_momentum: f64,
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
pub(crate) struct CoinMarketCapResponseData {
    pub data: CoinMarketCapData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinMarketCapData {
    pub points: Vec<CoinMarketCapPoint>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CoinMarketCapPoint {
    pub marketCap: f64,
    pub stableValue: f64,
    pub btcValue: f64,
    pub ethValue: f64,
    pub volume: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M2DataPoint {
    #[serde(rename = "Date")]
    pub date: String,
    
    #[serde(rename = "Global_M2")]
    pub global_m2: Option<f64>,
    
    #[serde(rename = "US_M2")]
    pub us_m2: Option<f64>,
    
    #[serde(rename = "Euro_M2")]
    pub euro_m2: Option<f64>,
    
    #[serde(rename = "China_M2")]
    pub china_m2: Option<f64>,
    
    #[serde(rename = "Japan_M2")]
    pub japan_m2: Option<f64>,
    
    #[serde(rename = "US_M2_Pct")]
    pub us_m2_pct: Option<f64>,
    
    #[serde(rename = "Euro_M2_Pct")]
    pub euro_m2_pct: Option<f64>,
    
    #[serde(rename = "China_M2_Pct")]
    pub china_m2_pct: Option<f64>,
    
    #[serde(rename = "Japan_M2_Pct")]
    pub japan_m2_pct: Option<f64>,
    
    #[serde(rename = "Euro_M2_Local")]
    pub euro_m2_local: Option<f64>,
    
    #[serde(rename = "China_M2_Local")]
    pub china_m2_local: Option<f64>,
    
    #[serde(rename = "Japan_M2_Local")]
    pub japan_m2_local: Option<f64>,
    
    #[serde(rename = "Global_M2_Local")]
    pub global_m2_local: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalM2Data {
    pub data_points: Vec<M2DataPoint>,
}