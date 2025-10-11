use anyhow::{Context, Result};
use crate::client::Web2Client;
use crate::models::{
    FearGreedIndex, FearGreedResponse, FredIndicator, FredResponse
};

pub struct MacroDataFetcher<'a> {
    client: &'a Web2Client,
}

impl<'a> MacroDataFetcher<'a> {
    pub fn new(client: &'a Web2Client) -> Self {
        Self { client }
    }

    pub async fn fetch_fear_greed_index(&self) -> Result<FearGreedIndex> {
        let url = "https://api.alternative.me/fng/?limit=1";
        
        let response: FearGreedResponse = self.client
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
            self.client.fred_api_key()
        );

        let response: FredResponse = self.client
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
}