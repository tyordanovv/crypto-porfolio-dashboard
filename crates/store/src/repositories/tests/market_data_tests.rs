use crate::{models::market_data_db::MarketDataDB, repositories::market_data_repository::MarketDataRepo};

use super::establish_test_pool;
use chrono::NaiveDate;

fn create_market_data(symbol: &str, date: (i32, u32, u32), price: f64) -> MarketDataDB {
    MarketDataDB {
        asset_symbol: symbol.to_string(),
        timestamp: NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap(),
        price_usd: price,
        volume_usd: Some(35_000_000_000.0),
        market_cap_usd: Some(1_300_000_000_000.0),
        dominance: Some(51.2),
    }
}

#[test]
fn test_insert_market_data() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_market_data("BTC", (2024, 10, 1), 68_000.0);
    let inserted = MarketDataRepo::insert(&mut conn, &rec).expect("insert failed");
    assert_eq!(inserted, 1);
}

#[test]
fn test_latest_market_data() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec1 = create_market_data("BTC", (2024, 10, 1), 68_000.0);
    let rec2 = create_market_data("BTC", (2024, 10, 2), 69_500.0);

    MarketDataRepo::insert(&mut conn, &rec1).unwrap();
    MarketDataRepo::insert(&mut conn, &rec2).unwrap();

    let latest = MarketDataRepo::latest_for_asset(&mut conn, "BTC").unwrap().unwrap();
    assert_eq!(latest.timestamp, rec2.timestamp);
    assert_eq!(latest.price_usd, rec2.price_usd);
}

#[test]
fn test_range_market_data() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec1 = create_market_data("BTC", (2024, 10, 1), 68_000.0);
    let rec2 = create_market_data("BTC", (2024, 10, 2), 69_500.0);

    MarketDataRepo::insert(&mut conn, &rec1).unwrap();
    MarketDataRepo::insert(&mut conn, &rec2).unwrap();

    let range = MarketDataRepo::range_for_asset(&mut conn, "BTC", rec1.timestamp, rec2.timestamp).unwrap();
    assert_eq!(range.len(), 2);
    assert_eq!(range[0].timestamp, rec1.timestamp);
    assert_eq!(range[1].timestamp, rec2.timestamp);
}

#[test]
fn test_upsert_market_data() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_market_data("BTC", (2024, 10, 1), 68_000.0);
    MarketDataRepo::insert(&mut conn, &rec).unwrap();

    // Update price for the same timestamp
    let updated = create_market_data("BTC", (2024, 10, 1), 70_000.0);
    MarketDataRepo::insert(&mut conn, &updated).unwrap();

    let fetched = MarketDataRepo::range_for_asset(&mut conn, "BTC", rec.timestamp, rec.timestamp).unwrap();
    assert_eq!(fetched.len(), 1);
    assert_eq!(fetched[0].price_usd, 70_000.0);
}