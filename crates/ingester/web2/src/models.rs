use serde::{Deserialize, Serialize};

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
    #[serde(rename = "marketCap")]
    pub market_cap: f64,
    #[serde(rename = "stableValue")]
    pub stable_value: f64,
    #[serde(rename = "btcValue")]
    pub btc_value: f64,
    #[serde(rename = "ethValue")]
    pub eth_value: f64,
    pub volume: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct M2DataPoint {
    pub country: String,
    pub iso_code: String,
    pub currency: String,
    pub date: String,
    pub m2: f64,
}

#[derive(Debug, Deserialize)]
pub struct FxEmpireM2Point {
    #[serde(rename = "formattedDate")]
    pub formatted_date: String,
    #[serde(rename = "close")]
    pub value: f64,
}