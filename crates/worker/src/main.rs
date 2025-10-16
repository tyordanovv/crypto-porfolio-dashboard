mod daily_ingestion;
mod montly_ingestion;
mod config;
mod framework;
mod util;

use anyhow::Result;
use dotenvy::dotenv;
use telemetry::setup_observability;
use crate::{
    config::{DailyWorkerConfig, MontlyWorkerConfig}, daily_ingestion::DailyIngestionJob, framework::{FixedIntervalScheduler, IngestionWorker, MonthlyScheduler}, montly_ingestion::MonthlyIngestionJob
};


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    setup_observability();
    println!("Starting ingestion workers");

    let fred_api_key = std::env::var("FRED_API_KEY")?;

    // --- Daily Job ---
    let daily_job = DailyIngestionJob::new(fred_api_key.clone(), DailyWorkerConfig::default());
    let daily_scheduler = FixedIntervalScheduler::new(std::time::Duration::from_secs(24 * 60 * 60));
    let daily_worker = IngestionWorker::new(daily_job, daily_scheduler, 3, std::time::Duration::from_secs(60));

    // --- Monthly Job ---
    let monthly_job = MonthlyIngestionJob::new(fred_api_key.clone(), MontlyWorkerConfig::default());
    let monthly_scheduler = MonthlyScheduler;
    let monthly_worker = IngestionWorker::new(monthly_job, monthly_scheduler, 3, std::time::Duration::from_secs(60));

    // Run both concurrently
    tokio::join!(
        daily_worker.run(),
        monthly_worker.run(),
    );

    Ok(())
}