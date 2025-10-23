use chrono::NaiveDate;

use crate::{models::market_metrics_db::MarketMetricDataDB, repositories::{market_metrics_repository::MarketMetricRepo, tests::establish_test_pool}};


fn create_record(date: (i32, u32, u32), value: f64) -> MarketMetricDataDB {
    MarketMetricDataDB {
        name: "RSI".to_string(),
        timestamp: NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap(),
        value: Some(value),
        source: Some("tradingview".to_string()),
    }
}

#[tokio::test]
async fn test_insert_indicator() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_record((2024, 10, 1), 55.3);
    let inserted = MarketMetricRepo::insert(&mut conn, &rec).await.unwrap();
    assert_eq!(inserted, 1);
}

#[tokio::test]
async fn test_latest_indicator() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec1 = create_record((2024, 10, 1), 55.3);
    let rec2 = create_record((2024, 10, 2), 60.1);

    MarketMetricRepo::insert(&mut conn, &rec1).await.unwrap();
    MarketMetricRepo::insert(&mut conn, &rec2).await.unwrap();

    let latest = MarketMetricRepo::latest_n(&mut conn, domain::MarketSymbol::BtcDominance, 1).await.unwrap();
    assert_eq!(latest.get(0).unwrap().timestamp, rec2.timestamp);
    assert_eq!(latest.get(0).unwrap().value, rec2.value);
}

#[tokio::test]
async fn test_range_indicators() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec1 = create_record((2024, 10, 1), 55.3);
    let rec2 = create_record((2024, 10, 2), 60.1);

    MarketMetricRepo::insert(&mut conn, &rec1).await.unwrap();
    MarketMetricRepo::insert(&mut conn, &rec2).await.unwrap();

    let range = MarketMetricRepo::range(&mut conn, "RSI", rec1.timestamp, rec2.timestamp).await.unwrap();
    assert_eq!(range.len(), 2);
    assert_eq!(range[0].timestamp, rec1.timestamp);
    assert_eq!(range[1].timestamp, rec2.timestamp);
}

#[tokio::test]
async fn test_upsert_indicator() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_record((2024, 10, 1), 55.3);
    MarketMetricRepo::insert(&mut conn, &rec).await.unwrap();

    // update the value for the same timestamp
    let updated = create_record((2024, 10, 1), 58.0);
    MarketMetricRepo::insert(&mut conn, &updated).await.unwrap();

    let fetched = MarketMetricRepo::range(&mut conn, "RSI", rec.timestamp, rec.timestamp).await.unwrap();
    assert_eq!(fetched.len(), 1);
    assert_eq!(fetched[0].value, Some(58.0));
}
