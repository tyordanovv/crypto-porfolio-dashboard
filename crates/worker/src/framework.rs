use std::time::Duration;

use anyhow::Result;
use chrono::{Utc};

use crate::util::sleep_until_next_month;

#[async_trait::async_trait]
pub trait IngestionJob: Send + Sync {
    type Output: std::fmt::Debug + Send;

    async fn fetch_all(&self) -> Result<Self::Output>;
    async fn store(&self, data: Self::Output) -> Result<()>;
    fn name(&self) -> &'static str;
}

pub struct IngestionWorker<J: IngestionJob, S: Scheduler> {
    job: J,
    scheduler: S,
    max_retries: u32,
    retry_delay: Duration,
}

impl<J: IngestionJob, S: Scheduler> IngestionWorker<J, S> {
    pub fn new(job: J, scheduler: S, max_retries: u32, retry_delay: Duration) -> Self {
        Self { job, scheduler, max_retries, retry_delay }
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Starting {} ingestion worker", self.job.name());

        // Run once on startup
        self.run_cycle().await?;

        // Schedule periodic execution
        loop {
            self.scheduler.wait_for_next().await;
            if let Err(e) = self.run_cycle().await {
                tracing::error!("Ingestion cycle failed: {:#}", e);
            }
        }
    }

    async fn run_cycle(&self) -> Result<()> {
        tracing::info!("Running {} cycle at {}", self.job.name(), Utc::now());
        let result = self.fetch_with_retry().await?;
        self.job.store(result).await?;
        tracing::info!("{} cycle completed successfully", self.job.name());
        Ok(())
    }

    async fn fetch_with_retry(&self) -> Result<J::Output> {
        let mut last_error = None;
        for attempt in 1..=self.max_retries {
            match self.job.fetch_all().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    tracing::warn!(
                        "Attempt {}/{} for {} failed: {}",
                        attempt,
                        self.max_retries,
                        self.job.name(),
                        e
                    );
                    last_error = Some(e);
                    if attempt < self.max_retries {
                        tokio::time::sleep(self.retry_delay).await;
                    }
                }
            }
        }
        Err(last_error.unwrap())
    }
}

#[async_trait::async_trait]
pub trait Scheduler: Send + Sync {
    async fn wait_for_next(&self);
}

pub struct FixedIntervalScheduler {
    interval: Duration,
}

impl FixedIntervalScheduler {
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }
}

pub struct MonthlyScheduler;

impl MonthlyScheduler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Scheduler for FixedIntervalScheduler {
    async fn wait_for_next(&self) {
        tokio::time::sleep(self.interval).await;
    }
}

#[async_trait::async_trait]
impl Scheduler for MonthlyScheduler {
    async fn wait_for_next(&self) {
        sleep_until_next_month().await;
    }
}