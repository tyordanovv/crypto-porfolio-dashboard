use super::establish_test_pool;
use crate::{models::NewStrategySignal, signal_repository::SignalsRepo};

fn create_signal(asset: &str, signal_type: &str, value: f64) -> NewStrategySignal {
    NewStrategySignal {
        asset_symbol: asset.to_string(),
        signal_type: signal_type.to_string(),
        value: Some(value),
        description: Some("Test signal".to_string()),
        source: Some("model_v1".to_string()),
    }
}

#[test]
fn test_insert_signal() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_signal("ETH", "BUY", 1.0);
    let inserted = SignalsRepo::insert(&mut conn, &rec).expect("insert failed");
    assert_eq!(inserted, 1);
}

#[test]
fn test_insert_and_get_latest_signal() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_signal("ETH", "BUY", 1.0);
    SignalsRepo::insert(&mut conn, &rec).unwrap();

    let fetched = SignalsRepo::latest_for_asset_and_type(&mut conn, "ETH", "BUY")
        .unwrap()
        .unwrap();

    assert_eq!(fetched.signal_type, "BUY");
    assert_eq!(fetched.value, Some(1.0));
    assert!(fetched.timestamp <= chrono::Utc::now().naive_utc());
}

#[test]
fn test_get_between_signals() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    // Insert multiple signals
    let rec1 = create_signal("ETH", "BUY", 1.0);
    let rec2 = create_signal("ETH", "BUY", 2.0);
    let rec3 = create_signal("ETH", "BUY", 3.0);

    SignalsRepo::insert(&mut conn, &rec1).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    SignalsRepo::insert(&mut conn, &rec2).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    SignalsRepo::insert(&mut conn, &rec3).unwrap();

    // Fetch all signals to get DB timestamps
    let all = SignalsRepo::get_between(
        &mut conn,
        "ETH",
        "BUY",
        chrono::Utc::now().naive_utc() - chrono::Duration::hours(1),
        chrono::Utc::now().naive_utc(),
    )
    .unwrap();

    assert!(all.len() > 3);

    // Use first and second timestamps as range
    let from = all[0].timestamp;
    let to = all[1].timestamp;

    let results = SignalsRepo::get_between(&mut conn, "ETH", "BUY", from, to).unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].value, Some(1.0));
    assert_eq!(results[1].value, Some(2.0));
}

#[test]
fn test_upsert_signal() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_signal("ETH", "BUY", 1.0);
    SignalsRepo::insert(&mut conn, &rec).unwrap();

    let fetched = SignalsRepo::latest_for_asset_and_type(&mut conn, "ETH", "BUY")
        .unwrap()
        .unwrap();

    use crate::models::StrategySignal;
    let upsert_rec = StrategySignal {
        timestamp: fetched.timestamp, // same timestamp
        ..fetched
    };

    let upsert_insertable = NewStrategySignal {
        asset_symbol: upsert_rec.asset_symbol.clone(),
        signal_type: upsert_rec.signal_type.clone(),
        value: Some(1.5),
        description: upsert_rec.description.clone(),
        source: upsert_rec.source.clone(),
    };

    SignalsRepo::insert(&mut conn, &upsert_insertable).unwrap();

    // Fetch again
    let updated = SignalsRepo::latest_for_asset_and_type(&mut conn, "ETH", "BUY")
        .unwrap()
        .unwrap();

    assert_eq!(updated.value, Some(1.5));
}
