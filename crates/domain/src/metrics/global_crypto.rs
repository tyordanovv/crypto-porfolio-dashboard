use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCryptoMarketData {
    pub total_market_cap_usd: f64,
    pub total_stable_cap_usd: f64,
    pub total_btc_cap_usd: f64,
    pub total_eth_cap_usd: f64,
    pub total_volume_24h_usd: f64,
}