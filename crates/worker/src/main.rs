mod daily_ingestion;
mod montly_ingestion;
mod config;

use anyhow::{Context, Result};
use daily_ingestion::DailyIngestionWorker;
use config::DailyWorkerConfig;
use telemetry::setup_observability;
use dotenvy::dotenv;

use crate::{config::MontlyWorkerConfig, montly_ingestion::MontlyIngestionWorker};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    setup_observability();
    println!("Starting");

    // Load configuration
    let fred_api_key = std::env::var("FRED_API_KEY")
        .context("FRED_API_KEY environment variable not set")?;

    let daily_config = DailyWorkerConfig::default();
    let montly_config = MontlyWorkerConfig::default();

    // Create and run worker
    let dayly_worker = DailyIngestionWorker::new(fred_api_key, daily_config);
    let montly_worker = MontlyIngestionWorker::new(fred_api_key, montly_config);
    dayly_worker.run().await?;
    montly_worker.run().await?;

    Ok(())
}