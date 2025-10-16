use chrono::{Utc, Datelike, TimeZone};
use tokio::time::sleep_until;

pub async fn sleep_until_next_month() {
    let now = Utc::now();
    let (next_year, next_month) = if now.month() == 12 {
        (now.year() + 1, 1)
    } else {
        (now.year(), now.month() + 1)
    };

    // Schedule at midnight UTC on 1st of next month
    let next_run = Utc
        .with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0)
        .unwrap();

    let duration = next_run - now;
    let until = tokio::time::Instant::now() + std::time::Duration::from_secs(duration.num_seconds() as u64);

    tracing::info!(
        "Next monthly run scheduled at {} (in {} hours)",
        next_run,
        duration.num_hours()
    );

    sleep_until(until).await;
}