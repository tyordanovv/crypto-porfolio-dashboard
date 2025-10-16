use std::time::Duration;

use web2::clients::MarketSymbol;

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub fred_series: Vec<String>,
    pub crypto_pairs: Vec<MarketSymbol>,
    pub market_series: Vec<MarketSymbol>,
    pub fetch_interval: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            fred_series: vec![
                "DFF".to_string(),      // Federal Funds Rate
                "T10Y2Y".to_string(),   // 10Y-2Y Treasury Spread
                "DEXUSEU".to_string(),  // USD/EUR Exchange Rate
                "CPIAUCSL".to_string(), // Consumer Price Index
                "DGS10".to_string(),    // 10-Year Treasury Constant Maturity Rate
                "DGS2".to_string(),     // 2-Year Treasury Constant Maturity Rate
                "M2SL".to_string(),     // M2 Money Stock
                "UNRATE".to_string(),   // Unemployment Rate
                "FEDFUNDS".to_string(), // Effective Federal Funds Rate
            ],
            crypto_pairs: vec![
                MarketSymbol::BtcUsd,
                MarketSymbol::EthUsd,
            ],
            market_series: vec![
                MarketSymbol::Gold,
                MarketSymbol::Oil,
                MarketSymbol::Sp500,
                MarketSymbol::Nasdaq,
                MarketSymbol::UsdIndex,

            ],
            fetch_interval: Duration::from_secs(24 * 60 * 60), // 24 hours
            max_retries: 3,
            retry_delay: Duration::from_secs(60),
        }
    }
}
