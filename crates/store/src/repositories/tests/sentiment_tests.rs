use chrono::NaiveDate;

use crate::{models::sentiment_db::SentimentDataDB, repositories::{sentiment_repository::SentimentRepo, tests::establish_test_pool}};


fn create_record(date: (i32, u32, u32), value: f64) -> SentimentDataDB {
    SentimentDataDB {
        name: "RSI".to_string(),
        timestamp: NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap(),
        value: Some(value),
        source: Some("tradingview".to_string()),
    }
}

#[test]
fn test_insert_indicator() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_record((2024, 10, 1), 55.3);
    let inserted = SentimentRepo::insert(&mut conn, &rec).expect("insert failed");
    assert_eq!(inserted, 1);
}

#[test]
fn test_latest_indicator() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec1 = create_record((2024, 10, 1), 55.3);
    let rec2 = create_record((2024, 10, 2), 60.1);

    SentimentRepo::insert(&mut conn, &rec1).unwrap();
    SentimentRepo::insert(&mut conn, &rec2).unwrap();

    let latest = SentimentRepo::latest(&mut conn, "RSI").unwrap().unwrap();
    assert_eq!(latest.timestamp, rec2.timestamp);
    assert_eq!(latest.value, rec2.value);
}

#[test]
fn test_range_indicators() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec1 = create_record((2024, 10, 1), 55.3);
    let rec2 = create_record((2024, 10, 2), 60.1);

    SentimentRepo::insert(&mut conn, &rec1).unwrap();
    SentimentRepo::insert(&mut conn, &rec2).unwrap();

    let range = SentimentRepo::range(&mut conn, "RSI", rec1.timestamp, rec2.timestamp).unwrap();
    assert_eq!(range.len(), 2);
    assert_eq!(range[0].timestamp, rec1.timestamp);
    assert_eq!(range[1].timestamp, rec2.timestamp);
}

#[test]
fn test_upsert_indicator() {
    let pool = establish_test_pool();
    let mut conn = pool.get().unwrap();

    let rec = create_record((2024, 10, 1), 55.3);
    SentimentRepo::insert(&mut conn, &rec).unwrap();

    // update the value for the same timestamp
    let updated = create_record((2024, 10, 1), 58.0);
    SentimentRepo::insert(&mut conn, &updated).unwrap();

    let fetched = SentimentRepo::range(&mut conn, "RSI", rec.timestamp, rec.timestamp).unwrap();
    assert_eq!(fetched.len(), 1);
    assert_eq!(fetched[0].value, Some(58.0));
}
