use chrono::NaiveDate;
use diesel::prelude::{Identifiable, Insertable, Queryable};

use crate::schema::sentiment_data;

#[derive(Debug, Clone, Queryable, Identifiable, Insertable)]
#[diesel(table_name = sentiment_data)]
#[diesel(primary_key(name, timestamp))]
pub struct SentimentDataDB {
    pub name: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}