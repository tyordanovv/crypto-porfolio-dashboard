use diesel::prelude::*;
use diesel::dsl::insert_into;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use crate::db::PgPooledConnection;
use crate::models::signal_db::{NewStrategySignalDB, StrategySignalDB};
use crate::schema::strategy_signals;
use chrono::NaiveDateTime;

/// Signals repository
pub struct SignalsRepo;

impl SignalsRepo {
    pub fn insert(conn: &mut PgPooledConnection, rec: &NewStrategySignalDB) -> Result<usize, DieselError> {
        insert_into(strategy_signals::table)
            .values(rec)
            .on_conflict((strategy_signals::asset_symbol, strategy_signals::signal_type, strategy_signals::timestamp))
            .do_update()
            .set((
                strategy_signals::value.eq(excluded(strategy_signals::value)),
                strategy_signals::description.eq(excluded(strategy_signals::description)),
                strategy_signals::source.eq(excluded(strategy_signals::source)),
            ))
            .execute(conn)
    }
    
    pub fn latest_for_asset_and_type(
        conn: &mut PgPooledConnection,
        asset: &str,
        signal_type_str: &str,
    ) -> Result<Option<StrategySignalDB>, DieselError> {
        strategy_signals::table
            .filter(strategy_signals::asset_symbol.eq(asset))
            .filter(strategy_signals::signal_type.eq(signal_type_str))
            .order(strategy_signals::timestamp.desc())
            .first::<StrategySignalDB>(conn)
            .optional()
    }

    pub fn get_between(
        conn: &mut PgPooledConnection,
        asset: &str,
        signal_type_str: &str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<Vec<StrategySignalDB>, DieselError> {
        strategy_signals::table
            .filter(strategy_signals::asset_symbol.eq(asset))
            .filter(strategy_signals::signal_type.eq(signal_type_str))
            .filter(strategy_signals::timestamp.ge(from))
            .filter(strategy_signals::timestamp.le(to))
            .order(strategy_signals::timestamp.asc())
            .load::<StrategySignalDB>(conn)
    }
}