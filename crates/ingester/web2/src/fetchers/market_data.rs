use anyhow::{Context, Result};
use domain::{GlobalCryptoMarketData, MarketPrice, MarketSymbol};
use crate::clients::{Web2Client, YahooClient};
use crate::models::{CoinMarketCapPoint, CoinMarketCapResponseData};
use chrono::{DateTime, Utc};

pub struct MarketDataFetcher<'a> {
    http_client: &'a Web2Client,
    yahoo_client: &'a YahooClient,
}

impl<'a> MarketDataFetcher<'a> {
    pub fn new(
        http_client: &'a Web2Client,
        yahoo_client: &'a YahooClient
    ) -> Self {
        Self { http_client, yahoo_client }
    }

    pub async fn fetch_global_market_data(&self) -> Result<GlobalCryptoMarketData> {
        let url = "https://api.coinmarketcap.com/data-api/v4/global-metrics/quotes/historical?convertId=2781&range=30d";

        let response: CoinMarketCapResponseData = self.http_client
            .http()
            .get(url)
            .send()
            .await
            .context("Failed to fetch global market data")?
            .json()
            .await
            .context("Failed to parse global market data")?;
        
        let mut points_sorted: Vec<(u64, &CoinMarketCapPoint)> = response.data
            .points
            .iter()
            .filter_map(|p| {
                let ts = p.timestamp.parse::<u64>().ok()?;
                Some((ts, p))
            })
            .collect();

        // sort by timestamp descending
        points_sorted.sort_by(|a, b| b.0.cmp(&a.0));

        // pick the second largest
        let last_full_day = points_sorted.get(1).map(|(_, p)| *p).unwrap(); // TODO handle error

        Ok(GlobalCryptoMarketData {
            total_market_cap_usd: last_full_day.market_cap,
            total_stable_cap_usd: last_full_day.stable_value,
            total_btc_cap_usd: last_full_day.btc_value,
            total_eth_cap_usd: last_full_day.eth_value,
            total_volume_24h_usd: last_full_day.volume
        })
    }

    pub async fn fetch_multiple_crypto_prices(
        &self,
        date: DateTime<Utc>,
        coins: Vec<MarketSymbol>,
    ) -> Vec<(MarketSymbol, Result<MarketPrice>)> {
        let futures = coins.iter().map(|symbol| {
            let symbol_owned = symbol.clone();
            async move {
                let result = self.yahoo_client
                    .fetch_market_data(date, &symbol_owned)
                    .await;

                (symbol_owned, result)
            }
        });

        futures::future::join_all(futures).await
    }
}