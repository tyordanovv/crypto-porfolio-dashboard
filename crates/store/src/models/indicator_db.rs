use chrono::NaiveDate;
use diesel::prelude::{Insertable, Queryable};

use crate::schema::indicators;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = indicators)]
pub struct IndicatorDB {
    pub name: String,
    pub category: String,
    pub timestamp: NaiveDate,
    pub value: Option<f64>,
    pub source: Option<String>,
}