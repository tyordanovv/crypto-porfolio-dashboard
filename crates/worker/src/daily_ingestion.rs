use anyhow::Result;
use chrono::Utc;
use tokio::time::{interval_at, Instant};
use web2::{FearGreedIndex, FredIndicator, GlobalCryptoMarketData, GlobalM2Data, MacroDataFetcher, MarketDataFetcher, clients::{MarketSymbol, Web2Client, YahooDataFetcher}, models::MarketPrice};

use crate::config::WorkerConfig;

pub struct DailyIngestionWorker {
    http_client: Web2Client,
    yahoo_client: YahooDataFetcher,
    config: WorkerConfig,
}

impl DailyIngestionWorker {
    pub fn new(fred_api_key: String, config: WorkerConfig) -> Self {
        let http_client = Web2Client::new(fred_api_key);
        let yahoo_client  = YahooDataFetcher::new();
        Self { http_client, yahoo_client, config }
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Starting daily ingestion worker");
        
        // Run immediately on startup
        self.run_ingestion_cycle().await?;

        // Schedule daily fetches
        let mut interval = interval_at(
            Instant::now() + self.config.fetch_interval,
            self.config.fetch_interval,
        );

        loop {
            interval.tick().await;
            if let Err(e) = self.run_ingestion_cycle().await {
                tracing::error!("Ingestion cycle failed: {:#}", e);
            }
        }
    }

    async fn run_ingestion_cycle(&self) -> Result<()> {
        tracing::info!("Starting ingestion cycle at {}", Utc::now());

        let result = self.fetch_with_retry().await?;

        self.store_data(result).await?;

        tracing::info!("Ingestion cycle completed successfully");
        Ok(())
    }

    async fn fetch_with_retry(&self) -> Result<IngestionResult> {
        let mut last_error = None;

        for attempt in 1..=self.config.max_retries {
            match self.fetch_all_data().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    tracing::warn!(
                        "Attempt {}/{} failed: {}",
                        attempt,
                        self.config.max_retries,
                        e
                    );
                    last_error = Some(e);

                    if attempt < self.config.max_retries {
                        tokio::time::sleep(self.config.retry_delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }

    async fn fetch_all_data(&self) -> Result<IngestionResult> {
        let macro_fetcher = MacroDataFetcher::new(&self.http_client, &self.yahoo_client);
        let market_fetcher = MarketDataFetcher::new(&self.http_client, &self.yahoo_client);

        // Fetch macro data
        let fear_greed = macro_fetcher.fetch_fear_greed_index().await.ok();

        let fred_series_refs: Vec<&str> = 
            self.config.fred_series.iter().map(|s| s.as_str()).collect();
        
        let fred_indicators = macro_fetcher
            .fetch_multiple_fred_indicators(&fred_series_refs)
            .await;

        // let global_m2_data = match macro_fetcher.fetch_global_m2_data().await {
        //     Ok(data) => data,
        //     Err(e) => {
        //         tracing::error!("Failed to fetch global M2 data: {:?}", e);
        //         return Err(e);
        //     }
        // };

        // Fetch market data
        let now = Utc::now();

        let combined: Vec<MarketSymbol> = self.config.crypto_pairs
            .iter()
            .chain(self.config.market_series.iter())
            .cloned()
            .collect();

        let crypto_prices = market_fetcher
            .fetch_multiple_crypto_prices(now, combined)
            .await;

        let global_crypto_data = market_fetcher.fetch_global_market_data().await?;

        // TODO compute BTC_dominance, ETH_dominance, BTC_stable_ratio, BTC_return_7d, BTC_return_30d, BTC_return_90d, BTC_volatility, BTC_momentum

        Ok(IngestionResult {
            timestamp: Utc::now(),
            fear_greed,
            fred_indicators,
            crypto_prices,
            global_crypto_data,
            // global_m2_data
        })
    }

    async fn store_data(&self, result: IngestionResult) -> Result<()> {
        // TODO: Get database connection from pool
        // let mut conn = get_db_connection()?;

        // Store Fear & Greed Index
        if let Some(fg) = result.fear_greed {
            tracing::info!(
                "Fear & Greed Index: {} ({})",
                fg.value,
                fg.classification
            );
        }

        // Store FRED indicators
        for (series_id, result) in result.fred_indicators {
            match result {
                Ok(indicator) => {
                    tracing::info!(
                        "FRED {}: {} ({})",
                        indicator.series_id,
                        indicator.value,
                        indicator.date
                    );
                    // TODO: Implement repository insert
                    // MacroIndicatorRepo::insert_fred(&mut conn, &indicator)?;
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch {}: {}", series_id, e);
                }
            }
        }

        // Store crypto prices
        for (symbol, result) in result.crypto_prices {
            match result {
                Ok(price) => {
                    tracing::info!(
                        "{}: ${:.2} 7d: ${:.2}, 30d: ${:.2}, 90d: ${:.2}, vol24h: ${:.0}",
                        price.symbol,
                        price.price_usd,
                        price.price_usd_7d_ago,
                        price.price_usd_30d_ago,
                        price.price_usd_90d_ago,
                        price.volume_24h_usd
                    );
                    // TODO: Implement repository insert
                    // MarketDataRepo::insert(&mut conn, &price)?;
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch {}: {}", symbol.as_str(), e);
                }
            }
        }

        // Store global market data
        tracing::info!(
            "Total Market Cap: ${:.2}B, Total Stable Coin Cap: ${:.2}B, Total Volume: ${}, BTC Cap: ${:.2}B, ETH Cap: ${:.2}B",
            result.global_crypto_data.total_market_cap_usd / 1_000_000_000.0,
            result.global_crypto_data.total_stable_cap_usd / 1_000_000_000.0,
            result.global_crypto_data.total_volume_24h_usd,
            result.global_crypto_data.total_btc_cap_usd / 1_000_000_000.0,
            result.global_crypto_data.total_eth_cap_usd / 1_000_000_000.0
        );
        // TODO: Implement repository insert
        // GlobalMarketDataRepo::insert(&mut conn, &result.global_data)?;

        Ok(())
    }
}

#[derive(Debug)]
struct IngestionResult {
    timestamp: chrono::DateTime<Utc>,
    fear_greed: Option<FearGreedIndex>,
    fred_indicators: Vec<(String, Result<FredIndicator>)>,
    crypto_prices: Vec<(MarketSymbol, Result<MarketPrice>)>,
    global_crypto_data: GlobalCryptoMarketData,
    // global_m2_data: GlobalM2Data,
}