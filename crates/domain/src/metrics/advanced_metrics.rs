use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{MarketPrice, MarketSymbol, metrics::{global_crypto::GlobalCryptoMarketData, utils}};

#[derive(Debug, Deserialize)]
pub struct AdvancedMetrics{
    pub timestamp: DateTime<Utc>,
    pub btc_dominance: f64,
    pub eth_dominance: f64,
    pub stablecoin_dominance: f64,
    pub btc_stable_ratio: f64,
    pub btc_return_7d: f64,
    pub btc_return_30d: f64,
    pub btc_return_90d: f64,
}

impl AdvancedMetrics {
    pub fn compute(
        crypto_prices: &[(MarketSymbol, anyhow::Result<MarketPrice>)],
        global_data: &GlobalCryptoMarketData,
    ) -> Self {
        let btc_prices: MarketPrice = crypto_prices
            .iter()
            .find_map(|(symbol, res)| {
                if symbol.is_btc() {
                    res.as_ref().ok().cloned()
                } else {
                    None
                }
            })
            .expect("no BTC price found");

        Self {
            timestamp: Utc::now(),
            btc_dominance: utils::btc_dominance(global_data),
            eth_dominance: utils::eth_dominance(global_data),
            stablecoin_dominance: utils::stablecoin_dominance(global_data),
            btc_stable_ratio: utils::btc_stable_ratio(global_data),
            btc_return_7d: utils::return_over_time(btc_prices.price_usd, btc_prices.price_usd_7d_ago),
            btc_return_30d: utils::return_over_time(btc_prices.price_usd, btc_prices.price_usd_30d_ago),
            btc_return_90d: utils::return_over_time(btc_prices.price_usd, btc_prices.price_usd_90d_ago),
        }
    }
}