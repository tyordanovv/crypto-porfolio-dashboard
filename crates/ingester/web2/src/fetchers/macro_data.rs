use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use crate::{clients::{MarketSymbol, Web2Client, YahooDataFetcher}, models::{
    FearGreedIndex, FearGreedResponse, FredIndicator, FredResponse, GlobalM2Data, M2DataPoint, MarketPrice
}};

pub struct MacroDataFetcher<'a> {
    http_client: &'a Web2Client,
    yahoo_client: &'a YahooDataFetcher,
}

impl<'a> MacroDataFetcher<'a> {
    pub fn new(
        http_client: &'a Web2Client,
        yahoo_client: &'a YahooDataFetcher,
    ) -> Self {
        Self { http_client, yahoo_client }
    }

    pub async fn fetch_fear_greed_index(&self) -> Result<FearGreedIndex> {
        let url = "https://api.alternative.me/fng/?limit=1";
        
        let response: FearGreedResponse = self.http_client
            .http()
            .get(url)
            .send()
            .await
            .context("Failed to fetch Fear & Greed Index")?
            .json()
            .await
            .context("Failed to parse Fear & Greed response")?;

        let item = response
            .data
            .first()
            .context("No Fear & Greed data available")?;

        Ok(FearGreedIndex {
            value: item.value.parse().context("Invalid value")?,
            classification: item.value_classification.clone(),
            timestamp: item.timestamp.parse().context("Invalid timestamp")?,
        })
    }

    pub async fn fetch_fred_indicator(&self, series_id: &str) -> Result<FredIndicator> {
        let url = format!(
            "https://api.stlouisfed.org/fred/series/observations\
             ?series_id={}&api_key={}&file_type=json&sort_order=desc&limit=1",
            series_id,
            self.http_client.fred_api_key()
        );

        let response: FredResponse = self.http_client
            .http()
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch FRED series {}", series_id))?
            .json()
            .await
            .context("Failed to parse FRED response")?;

        let obs = response
            .observations
            .first()
            .context("No observations available")?;

        Ok(FredIndicator {
            series_id: series_id.to_string(),
            value: obs.value.parse().context("Invalid value")?,
            date: obs.date.clone(),
        })
    }

    pub async fn fetch_multiple_fred_indicators(
        &self,
        series_ids: &[&str],
    ) -> Vec<(String, Result<FredIndicator>)> {
        let futures = series_ids
            .iter()
            .map(|id| async move {
                let result = self.fetch_fred_indicator(id).await;
                (id.to_string(), result)
            });

        futures::future::join_all(futures).await
    }

    // pub async fn fetch_global_m2_data(&self) -> Result<GlobalM2Data> {
    //     // First try direct access (might work for public endpoints)
    //     Ok(() as GlobalM2Data)
    // }
    
    pub async fn fetch_multiple_market_prices(
        &self,
        date: DateTime<Utc>,
        symbols: &Vec<MarketSymbol>,
    ) -> Vec<(MarketSymbol, Result<MarketPrice>)> {
        let futures = symbols.iter().map(|symbol| {
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