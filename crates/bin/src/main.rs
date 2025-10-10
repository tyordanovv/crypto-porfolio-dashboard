use telemetry::setup_observability;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    setup_observability();
    println!("Starting");
    // Keep main alive forever
    loop {
        sleep(Duration::from_secs(60)).await;
        println!("Heartbeat - still running...");
    }
}