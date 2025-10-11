use diesel::prelude::{Identifiable, Insertable, Queryable};
use uuid::Uuid;

use crate::schema::strategy_signals;

//
// StrategySignal
//
#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = strategy_signals)]
#[diesel(primary_key(id))]
pub struct StrategySignalDB {
    pub id: Uuid,
    pub asset_symbol: String,
    pub timestamp: chrono::NaiveDateTime,
    pub signal_type: String,
    pub value: Option<f64>,
    pub description: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = strategy_signals)]
pub struct NewStrategySignalDB {
    pub asset_symbol: String,
    pub signal_type: String,
    pub value: Option<f64>,
    pub description: Option<String>,
    pub source: Option<String>,
}