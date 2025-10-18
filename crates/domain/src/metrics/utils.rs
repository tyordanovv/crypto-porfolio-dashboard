use crate::GlobalCryptoMarketData;

/// Calculate BTC dominance as % of total market cap.
pub fn btc_dominance(global: &GlobalCryptoMarketData) -> f64 {
    global.total_btc_cap_usd / global.total_market_cap_usd
}

/// Calculate ETH dominance as % of total market cap.
pub fn eth_dominance(global: &GlobalCryptoMarketData) -> f64 {
    global.total_eth_cap_usd / global.total_market_cap_usd
}

/// Stablecoin dominance as % of total market cap.
pub fn stablecoin_dominance(global: &GlobalCryptoMarketData) -> f64 {
    global.total_stable_cap_usd / global.total_market_cap_usd
}

/// BTC to Stablecoin market-cap ratio.
pub fn btc_stable_ratio(global: &GlobalCryptoMarketData) -> f64 {
    if global.total_stable_cap_usd == 0.0 {
        return 0.0;
    }
    global.total_btc_cap_usd / global.total_stable_cap_usd
}

/// Compute return given historical prices.
pub fn return_over_time(recent: f64, past: f64) -> f64 {
    (recent - past) / past
}