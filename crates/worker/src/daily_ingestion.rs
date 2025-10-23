use anyhow::Result;
use chrono::Utc;
use domain::{AdvancedMetrics, FearGreedIndexData, FredIndexData, GlobalCryptoMarketData, MarketPrice, MarketSymbol, native_date_from_str};
use store::{db::PgPool, models::{market_data_db::MarketDataDB, market_metrics_db::MarketMetricDataDB}, repositories::{market_data_repository::MarketDataRepo, market_metrics_repository::MarketMetricRepo}};
use tracing::{info, warn};
use web2::{ MacroDataFetcher, MarketDataFetcher, clients::{Web2Client, YahooClient} };
use crate::{config::DailyWorkerConfig, framework::IngestionJob};

pub struct DailyIngestionJob {
    http_client: Web2Client,
    yahoo_client: YahooClient,
    config: DailyWorkerConfig,
    db_pool: PgPool
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
    pub fn new(fred_api_key: String, config: DailyWorkerConfig, db_pool: PgPool) -> Self {
        Self {
            http_client: Web2Client::new(fred_api_key),
            yahoo_client: YahooClient::new(),
            config,
            db_pool,
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
        use tracing::{info, warn};

        // Acquire connection asynchronously if your pool is async
        let mut conn = self.db_pool.get()
            .map_err(|e| anyhow::anyhow!("Failed to get DB connection: {e}"))?;

        let date = result.timestamp.date_naive();

        // === 1. Fear & Greed Index ===
        let fg_value: f64 = result.fear_greed.value.parse().unwrap_or_default();
        if let Err(e) = MarketMetricRepo::insert(
            &mut conn,
            &MarketMetricDataDB {
                name: MarketSymbol::FearGreedIndex.as_str().into(),
                timestamp: date,
                value: Some(fg_value),
                source: Some("alternative.me".into()),
            },
        ).await {
            warn!("Failed to persist Fear & Greed Index: {e}");
        }

        // === 2. FRED indicators ===
        for (series_id, res) in result.fred_indicators {
            match res {
                Ok(data) => {
                    if let Err(e) = MarketMetricRepo::insert(
                        &mut conn,
                        &MarketMetricDataDB {
                            name: data.series_id.clone(),
                            timestamp: native_date_from_str(&data.date),
                            value: Some(data.value),
                            source: Some("fred".into()),
                        },
                    ).await {
                        warn!("Failed to persist FRED {}: {}", series_id, e);
                    }
                }
                Err(e) => warn!("Failed to fetch FRED {}: {}", series_id, e),
            }
        }

        // === 3. Crypto & market assets ===
        for (symbol, res) in result.crypto_prices {
            match res {
                Ok(price) => {
                    let rec = MarketDataDB {
                        asset_symbol: symbol.as_str().to_string(),
                        timestamp: date,
                        price_usd: price.price_usd,
                        volume_usd: Some(price.volume_24h_usd as f64),
                        market_cap_usd: None,
                        dominance: None,
                    };
                    if let Err(e) = MarketDataRepo::insert(&mut conn, &rec).await {
                        warn!("Failed to persist {}: {}", symbol.as_str(), e);
                    }
                }
                Err(e) => warn!("Failed to fetch {}: {}", symbol.as_str(), e),
            }
        }

        // === 4. Global crypto data ===
        let g = result.global_crypto_data;
        let metrics = vec![
            (MarketSymbol::GlobalTotalMarketCapUsd, g.total_market_cap_usd),
            (MarketSymbol::GlobalTotalStableCapUsd, g.total_stable_cap_usd),
            (MarketSymbol::GlobalTotalBtcCapUsd, g.total_btc_cap_usd),
            (MarketSymbol::GlobalTotalEthCapUsd, g.total_eth_cap_usd),
            (MarketSymbol::GlobalTotalVolume24hUsd, g.total_volume_24h_usd),
        ];

        for (metric, value) in metrics {
            if let Err(e) = MarketMetricRepo::insert(
                &mut conn,
                &MarketMetricDataDB {
                    name: metric.as_str().into(),
                    timestamp: date,
                    value: Some(value),
                    source: Some("coingecko".into()),
                },
            ).await {
                warn!("Failed to persist global metric {}: {}", metric.as_str(), e);
            }
        }

        // === 5. Advanced metrics ===
        let a = result.advanced_metrics;
        let adv_metrics = vec![
            (MarketSymbol::BtcDominance, a.btc_dominance),
            (MarketSymbol::EthDominance, a.eth_dominance),
            (MarketSymbol::StablecoinDominance, a.stablecoin_dominance),
            (MarketSymbol::BtcStableRatio, a.btc_stable_ratio),
            (MarketSymbol::BtcReturn7d, a.btc_return_7d),
            (MarketSymbol::BtcReturn30d, a.btc_return_30d),
            (MarketSymbol::BtcReturn90d, a.btc_return_90d),
        ];

        for (adv_metric, value) in adv_metrics {
            if let Err(e) = MarketMetricRepo::insert(
                &mut conn,
                &MarketMetricDataDB {
                    name: adv_metric.as_str().into(),
                    timestamp: a.timestamp.date_naive(),
                    value: Some(value),
                    source: Some("computed".into()),
                },
            ).await {
                warn!("Failed to persist advanced metric {}: {}", adv_metric.as_str(), e);
            }
        }

        info!("Daily data persisted successfully at {}", result.timestamp);
        Ok(())
    }
}
