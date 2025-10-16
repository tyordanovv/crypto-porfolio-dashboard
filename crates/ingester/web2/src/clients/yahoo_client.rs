use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use domain::utils::chrono_to_offset;
use yahoo_finance_api as yahoo;

use crate::models::MarketPrice;

#[derive(Debug, Clone)]
pub enum MarketSymbol {
    BtcUsd,
    EthUsd,
    Gold,
    Oil,
    Sp500,
    Nasdaq,
    UsdIndex,
}

impl MarketSymbol {
    pub fn as_yahoo_symbol(&self) -> &'static str {
        match self {
            MarketSymbol::BtcUsd => "BTC-USD",
            MarketSymbol::EthUsd => "ETH-USD",
            MarketSymbol::Gold => "GC=F",
            MarketSymbol::Oil => "CL=F",
            MarketSymbol::Sp500 => "^GSPC",
            MarketSymbol::Nasdaq => "^IXIC",
            MarketSymbol::UsdIndex => "DX-Y.NYB",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MarketSymbol::BtcUsd => "BTC",
            MarketSymbol::EthUsd => "ETH",
            MarketSymbol::Gold => "GOLD",
            MarketSymbol::Oil => "OIL",
            MarketSymbol::Sp500 => "SP500",
            MarketSymbol::Nasdaq => "NASDAQ",
            MarketSymbol::UsdIndex => "USD_INDEX",
        }
    }
}

pub struct YahooClient {
    provider: yahoo::YahooConnector,
}

impl YahooClient {
    pub fn new() -> Self {
        Self {
            provider: yahoo::YahooConnector::new().unwrap(),
        }
    }

    fn find_closest(&self, timestamps: &[i64], closes: &[f64], target: i64) -> f64 {
        timestamps
            .iter()
            .zip(closes.iter())
            .min_by_key(|(ts, _)| (*ts - target).abs())
            .map(|(_, close)| *close)
            .unwrap_or(0.0)
    }

    pub async fn fetch_market_data(
        &self,
        now: DateTime<Utc>,
        symbol: &MarketSymbol,
    ) -> Result<MarketPrice> {
        let start = chrono_to_offset(now - Duration::days(90));
        let now_offset = chrono_to_offset(now);

        let yahoo_symbol = symbol.as_yahoo_symbol();

        let response = self.provider 
            .get_quote_history(yahoo_symbol, start, now_offset)
            .await
            .with_context(|| format!("Failed to fetch Yahoo data for {}", yahoo_symbol))?;

        let quotes = response.quotes().context("No quotes found in Yahoo response")?;

        if quotes.is_empty() {
            anyhow::bail!("No quote data for symbol {}", yahoo_symbol);
        }

        let timestamps: Vec<i64> = quotes.iter().map(|q| q.timestamp).collect();
        let closes: Vec<f64> = quotes.iter().map(|q| q.close).collect();
        let volumes: Vec<u64> = quotes.iter().map(|q| q.volume).collect();

        let last_price = *closes.last().unwrap_or(&0.0);
        let last_volume = *volumes.last().unwrap_or(&0);

        let price_7d_ago = self.find_closest(&timestamps, &closes, (now - Duration::days(7)).timestamp());
        let price_30d_ago = self.find_closest(&timestamps, &closes, (now - Duration::days(30)).timestamp());
        let price_90d_ago = self.find_closest(&timestamps, &closes, (now - Duration::days(90)).timestamp());

        Ok(MarketPrice {
            symbol: symbol.as_str().to_string(),
            price_usd: last_price,
            price_usd_7d_ago: price_7d_ago,
            price_usd_30d_ago: price_30d_ago,
            price_usd_90d_ago: price_90d_ago,
            volume_24h_usd: last_volume,
        })
    }
}