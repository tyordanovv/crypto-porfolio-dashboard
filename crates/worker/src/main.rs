mod daily_ingestion;
mod config;

use anyhow::{Context, Result};
use daily_ingestion::DailyIngestionWorker;
use config::WorkerConfig;
use telemetry::setup_observability;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    setup_observability();
    println!("Starting");

    // Load configuration
    let fred_api_key = std::env::var("FRED_API_KEY")
        .context("FRED_API_KEY environment variable not set")?;

    let config = WorkerConfig::default();

    // Create and run worker
    let worker = DailyIngestionWorker::new(fred_api_key, config);
    worker.run().await?;

    Ok(())
}