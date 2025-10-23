use chrono::{DateTime, NaiveDate, Utc};
use time::OffsetDateTime;

pub fn current_timestamp_ms() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn normalize_symbol(coin_id: &str) -> String {
    let s: &str = match coin_id {
        "bitcoin" => "BTC",
        "ethereum" => "ETH",
        "sui" => "SUI",
        "solana" => "SOL",
        _ => coin_id,
    };
    s.to_uppercase()
}

pub fn chrono_to_offset(dt: DateTime<Utc>) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(dt.timestamp())
        .expect("Invalid timestamp conversion")
}

pub fn native_date_from_str(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .expect("Invalid date conversion")
}