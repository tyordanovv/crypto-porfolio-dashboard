use std::time::Duration;

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub fred_series: Vec<String>,
    pub crypto_pairs: Vec<(String, String)>, // (coin_id, symbol)
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
            ],
            crypto_pairs: vec![
                ("bitcoin".to_string(), "BTC-USD".to_string()),
                ("ethereum".to_string(), "ETH-USD".to_string()),
            ],
            fetch_interval: Duration::from_secs(24 * 60 * 60), // 24 hours
            max_retries: 3,
            retry_delay: Duration::from_secs(60),
        }
    }
}
