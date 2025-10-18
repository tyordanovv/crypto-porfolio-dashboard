use anyhow::Result;
use chrono::Utc;
use web2::{MacroDataFetcher, clients::{M2Country, Web2Client, YahooClient}, models::M2DataPoint};
use crate::config::MontlyWorkerConfig;
use super::framework::IngestionJob;

pub struct MonthlyIngestionJob {
    http_client: Web2Client,
    yahoo_client: YahooClient,
    config: MontlyWorkerConfig,
}

#[derive(Debug)]
pub struct MonthlyIngestionResult {
    timestamp: chrono::DateTime<Utc>,
    global_m2_data: Vec<(M2Country, Result<M2DataPoint>)>,
}

impl MonthlyIngestionJob {
    pub fn new(fred_api_key: String, config: MontlyWorkerConfig) -> Self {
        Self {
            http_client: Web2Client::new(fred_api_key),
            yahoo_client: YahooClient::new(),
            config,
        }
    }
}

#[async_trait::async_trait]
impl IngestionJob for MonthlyIngestionJob {
    type Output = MonthlyIngestionResult;
    fn name(&self) -> &'static str { "monthly" }

    async fn fetch_all(&self) -> Result<Self::Output> {
        let macro_fetcher = MacroDataFetcher::new(&self.http_client, &self.yahoo_client);
        let global_m2_data = macro_fetcher.fetch_global_m2_data(&self.config.m2_countries).await;
        Ok(MonthlyIngestionResult {
            timestamp: Utc::now(),
            global_m2_data,
        })
    }

    async fn store(&self, result: Self::Output) -> Result<()> {
        for (symbol, result) in result.global_m2_data {
            match result {
                Ok(m2_data) => tracing::info!( "M2 Data for {} {}: {}{:.2} on {}", m2_data.country, m2_data.iso_code, m2_data.currency, m2_data.m2, m2_data.date
                ),
                Err(e) => tracing::warn!("Failed to fetch {}: {}", symbol.as_str(), e),
            }
        }
        Ok(())
    }
}
