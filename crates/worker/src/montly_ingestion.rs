use anyhow::Result;
use chrono::Utc;
use web2::{GlobalM2Data, MacroDataFetcher, clients::{Web2Client, YahooClient}};
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
    global_m2_data: GlobalM2Data,
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
        let global_m2_data = macro_fetcher.fetch_global_m2_data().await?;
        Ok(MonthlyIngestionResult {
            timestamp: Utc::now(),
            global_m2_data,
        })
    }

    async fn store(&self, _result: Self::Output) -> Result<()> {
        // TODO implement storage
        Ok(())
    }
}
