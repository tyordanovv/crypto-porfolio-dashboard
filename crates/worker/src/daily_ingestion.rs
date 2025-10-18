use anyhow::Result;
use chrono::Utc;
use domain::{AdvancedMetrics, FearGreedIndexData, FredIndexData, GlobalCryptoMarketData, MarketPrice, MarketSymbol};
use web2::{ MacroDataFetcher, MarketDataFetcher, clients::{Web2Client, YahooClient} };
use crate::{config::DailyWorkerConfig, framework::IngestionJob};

pub struct DailyIngestionJob {
    http_client: Web2Client,
    yahoo_client: YahooClient,
    config: DailyWorkerConfig,
}

#[derive(Debug)]
pub struct DailyIngestionResult {
    timestamp: chrono::DateTime<Utc>,
    fear_greed: FearGreedIndexData,
    fred_indicators: Vec<(String, Result<FredIndexData>)>,
    crypto_prices: Vec<(MarketSymbol, Result<MarketPrice>)>,
    global_crypto_data: GlobalCryptoMarketData,
    advanced_metrics: AdvancedMetrics,
}

impl DailyIngestionJob {
    pub fn new(fred_api_key: String, config: DailyWorkerConfig) -> Self {
        Self {
            http_client: Web2Client::new(fred_api_key),
            yahoo_client: YahooClient::new(),
            config,
        }
    }
}

#[async_trait::async_trait]
impl IngestionJob for DailyIngestionJob {
    type Output = DailyIngestionResult;

    fn name(&self) -> &'static str { "daily" }

    async fn fetch_all(&self) -> Result<Self::Output> {
        let macro_fetcher = MacroDataFetcher::new(&self.http_client, &self.yahoo_client);
        let market_fetcher = MarketDataFetcher::new(&self.http_client, &self.yahoo_client);

        let fear_greed = macro_fetcher.fetch_fear_greed_index().await?;

        let fred_series_refs: Vec<&str> = self.config.fred_series.iter().map(|s| s.as_str()).collect();
        let fred_indicators = macro_fetcher.fetch_multiple_fred_indicators(&fred_series_refs).await;

        let now = Utc::now();
        let combined: Vec<MarketSymbol> = self.config.crypto_pairs
            .iter()
            .chain(self.config.market_series.iter())
            .cloned()
            .collect();
        let crypto_prices = market_fetcher.fetch_multiple_crypto_prices(now, combined).await;
        let global_crypto_data = market_fetcher.fetch_global_market_data().await?;

        let advanced_metrics = AdvancedMetrics::compute(&crypto_prices, &global_crypto_data);

        Ok(DailyIngestionResult {
            timestamp: Utc::now(),
            fear_greed,
            fred_indicators,
            crypto_prices,
            global_crypto_data,
            advanced_metrics
        })
    }

    async fn store(&self, result: Self::Output) -> Result<()> {
        // Logging placeholders (replace with repo inserts)
        tracing::info!("Ingestion Timestamp: {}", result.timestamp);
        tracing::info!("Fear & Greed Index: {} ({})", result.fear_greed.value, result.fear_greed.classification);

        for (series_id, result) in result.fred_indicators {
            match result {
                Ok(indicator) => tracing::info!(
                    "FRED {}: {} ({})", indicator.series_id, indicator.value, indicator.date
                ),
                Err(e) => tracing::warn!("Failed to fetch {}: {}", series_id, e),
            }
        }

        for (symbol, result) in result.crypto_prices {
            match result {
                Ok(price) => tracing::info!(
                    "{}: ${:.2}, vol24h: ${:.0}",
                    price.symbol.as_str(), price.price_usd, price.volume_24h_usd
                ),
                Err(e) => tracing::warn!("Failed to fetch {}: {}", symbol.as_str(), e),
            }
        }

        tracing::info!(
            "Advanced Metrics at {}: BTC Dominance: {:.2}%, ETH Dominance: {:.2}%, Stablecoin Dominance: {:.2}%, BTC/Stable Ratio: {:.2}, BTC 7d Return: {:.2}%, BTC 30d Return: {:.2}%, BTC 90d Return: {:.2}%",
            result.advanced_metrics.timestamp,
            result.advanced_metrics.btc_dominance,
            result.advanced_metrics.eth_dominance,
            result.advanced_metrics.stablecoin_dominance,
            result.advanced_metrics.btc_stable_ratio,
            result.advanced_metrics.btc_return_7d,
            result.advanced_metrics.btc_return_30d,
            result.advanced_metrics.btc_return_90d
        );

        tracing::info!(
            "Total Market Cap: ${:.2}B, BTC Cap: ${:.2}B, ETH Cap: ${:.2}B",
            result.global_crypto_data.total_market_cap_usd / 1_000_000_000.0,
            result.global_crypto_data.total_btc_cap_usd / 1_000_000_000.0,
            result.global_crypto_data.total_eth_cap_usd / 1_000_000_000.0
        );

        Ok(())
    }
}
