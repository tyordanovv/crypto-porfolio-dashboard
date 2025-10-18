use chrono::NaiveDate;
use diesel::prelude::{Identifiable, Insertable, Queryable};

use crate::schema::market_metrics;

#[derive(Debug, Clone, Queryable, Identifiable, Insertable)]
#[diesel(table_name = market_metrics)]
#[diesel(primary_key(name, timestamp))]
pub struct SentimentDataDB {
    pub name: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}