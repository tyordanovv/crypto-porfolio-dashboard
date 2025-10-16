use crate::config::{MontlyWorkerConfig};
use chrono::Utc;
use tokio::time::{interval_at, Instant};
use anyhow::Result;
use web2::{GlobalM2Data, MacroDataFetcher, clients::{Web2Client, YahooClient}};

pub struct MontlyIngestionWorker {
    config: MontlyWorkerConfig,
    http_client: Web2Client,
    yahoo_client: YahooClient,
}

impl MontlyIngestionWorker {
    pub fn new(fred_api_key: String, config: MontlyWorkerConfig) -> Self {
        let http_client = Web2Client::new(fred_api_key);
        let yahoo_client  = YahooClient::new();
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

    async fn fetch_with_retry(&self) -> Result<MontlyIngestionResult> {
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

    async fn fetch_all_data(&self) -> Result<MontlyIngestionResult> {
        let macro_fetcher = MacroDataFetcher::new(&self.http_client, &self.yahoo_client);

        let global_m2_data = macro_fetcher.fetch_global_m2_data().await?;

        Ok(MontlyIngestionResult {
            timestamp: Utc::now(),
            global_m2_data: global_m2_data,
        })
    }

    async fn store_data(&self, result: MontlyIngestionResult) -> Result<()> {
        // TODO: Get database connection from pool

        Ok(())
    }
}

#[derive(Debug)]
struct MontlyIngestionResult {
    timestamp: chrono::DateTime<Utc>,
    global_m2_data: GlobalM2Data,
}
