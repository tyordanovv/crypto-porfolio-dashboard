use anyhow::{Context, Result};
use crate::client::Web2Client;
use crate::models::{
    CoinMarketCapPoint, CoinMarketCapResponseData, CryptoPrice, GlobalMarketData, YahooChart
};
use chrono::{DateTime, Duration, Utc};

pub struct MarketDataFetcher<'a> {
    client: &'a Web2Client,
}

impl<'a> MarketDataFetcher<'a> {
    pub fn new(client: &'a Web2Client) -> Self {
        Self { client }
    }

    pub async fn fetch_crypto_price(
        &self,
        now: DateTime<Utc>,
        symbol: &str,
    ) -> Result<CryptoPrice> {
        let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1d&range=3mo", symbol);

        let resp: YahooChart = self.client
            .http()
            .get(&url)
            .send()
            .await
            .context("Failed to fetch Yahoo Finance data")?
            .json()
            .await
            .context("Failed to parse Yahoo Finance JSON")?;

        let chart_result = resp
            .chart
            .result
            .ok_or_else(|| anyhow::anyhow!("No chart result for symbol"))?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Empty chart result"))?;

        let timestamps = chart_result
            .timestamp
            .ok_or_else(|| anyhow::anyhow!("No timestamps in chart"))?;

        let quote = chart_result
            .indicators
            .quote
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No quote data"))?;

        let closes = quote.close;
        let volumes = quote.volume;

        if timestamps.len() != closes.len() || timestamps.len() != volumes.len() {
            return Err(anyhow::anyhow!("Length mismatch in Yahoo chart data"));
        }

        let last_price = closes.last().and_then(|c| *c).unwrap_or(0.0);
        let last_volume = volumes.last().and_then(|v| *v).unwrap_or(0.0);

        let price_7d_ago = self.find_closest(&timestamps, &closes, (now - Duration::days(7)).timestamp());
        let price_30d_ago = self.find_closest(&timestamps, &closes, (now - Duration::days(30)).timestamp());
        let price_90d_ago = self.find_closest(&timestamps, &closes, (now - Duration::days(90)).timestamp());

        Ok(CryptoPrice {
            symbol: symbol.to_string(),
            price_usd: last_price,
            price_usd_7d_ago: price_7d_ago,
            price_usd_30d_ago: price_30d_ago,
            price_usd_90d_ago: price_90d_ago,
            volume_24h_usd: last_volume,
        })
    }

    pub async fn fetch_global_market_data(&self) -> Result<GlobalMarketData> {
        let url = "https://api.coinmarketcap.com/data-api/v4/global-metrics/quotes/historical?convertId=2781&range=30d";

        let response: CoinMarketCapResponseData = self.client
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

        Ok(GlobalMarketData {
            total_market_cap_usd: last_full_day.marketCap,
            total_stable_cap_usd: last_full_day.stableValue,
            total_btc_cap_usd: last_full_day.btcValue,
            total_eth_cap_usd: last_full_day.ethValue,
            total_volume_24h_usd: last_full_day.volume
        })
    }

    pub async fn fetch_multiple_prices(
        &self,
        coins: &[(&str, &str)], // (coin_id, symbol)
    ) -> Vec<(String, Result<CryptoPrice>)> {
        let futures = coins
            .iter()
            .map(|(_, symbol)| async move {
                let now = Utc::now();
                let result = self.fetch_crypto_price(now, symbol).await;
                (symbol.to_string(), result)
            });

        futures::future::join_all(futures).await
    }

    fn find_closest(&self, timestamps: &[i64], closes: &[Option<f64>], target: i64) -> f64 {
        timestamps
            .iter()
            .zip(closes.iter())
            .min_by_key(|(ts, _)| (*ts - target).abs())
            .and_then(|(_, close)| *close)
            .unwrap_or(0.0)
    }
}