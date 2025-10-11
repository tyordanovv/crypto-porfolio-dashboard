use chrono::NaiveDate;
use diesel::prelude::*;
use crate::schema::market_data;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = market_data)]
#[diesel(primary_key(asset_symbol, timestamp))]
pub struct MarketDataDB {
    pub asset_symbol: String,
    pub timestamp: NaiveDate,
    pub price_usd: f64,
    pub volume_usd: Option<f64>,
    pub market_cap_usd: Option<f64>,
    pub dominance: Option<f64>,
}