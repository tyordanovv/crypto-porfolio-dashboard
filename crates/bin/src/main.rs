use std::sync::Arc;
use telemetry::setup_observability;
use tokio::time::{sleep, Duration};
use store::HotStore;
use detector::detect_spread;
use ingester::CetusIngester;
use dex_core::DexAdapter;
    
#[tokio::main]
async fn main() {
    setup_observability();
    println!("Starting arbitrage bot...");
    
    let store = Arc::new(HotStore::new());
    let store_clone = store.clone();

    // Ingest task
    tokio::spawn(async move {
        let ingester = CetusIngester { rpc_url: "https://rpc.sui.io".to_string() };
        loop {
            let prices = ingester.poll_prices().await;
            println!("Ingested {} prices", prices.len());
            for p in prices {
                store_clone.update(p);
            }
            sleep(Duration::from_secs(2)).await;
        }
    });

    // Detector task
    let store_clone2 = store.clone();
    tokio::spawn(async move {
        loop {
            let pair_prices = store_clone2.get_pair_prices((1, 2));
            if let Some(spread) = detect_spread(&pair_prices) {
                println!("🎯 Spread detected: {:?}", spread);
            } else {
                println!("⏳ No spread detected...");
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    // Keep main alive forever
    loop {
        sleep(Duration::from_secs(60)).await;
        println!("Heartbeat - still running...");
    }
}