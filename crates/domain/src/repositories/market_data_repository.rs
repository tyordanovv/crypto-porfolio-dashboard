use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::dsl::insert_into;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use crate::db::PgPooledConnection;
use crate::models::{ MarketData, NewMarketData };
use crate::schema::market_data;

/// MarketData repository
pub struct MarketDataRepo;

impl MarketDataRepo {
    pub fn insert(conn: &mut PgPooledConnection, rec: &NewMarketData) -> Result<usize, DieselError> {
        insert_into(market_data::table)
            .values(rec)
            .on_conflict((market_data::asset_symbol, market_data::timestamp))
            .do_update()
            .set((
                market_data::price_usd.eq(excluded(market_data::price_usd)),
                market_data::volume_usd.eq(excluded(market_data::volume_usd)),
                market_data::market_cap_usd.eq(excluded(market_data::market_cap_usd)),
                market_data::dominance.eq(excluded(market_data::dominance)),
            ))
            .execute(conn)
    }

    pub fn latest_for_asset(conn: &mut PgPooledConnection, symbol: &str) -> Result<Option<MarketData>, DieselError> {
        market_data::table
            .filter(market_data::asset_symbol.eq(symbol))
            .order(market_data::timestamp.desc())
            .first::<MarketData>(conn)
            .optional()
    }

    pub fn range_for_asset(
        conn: &mut PgPooledConnection,
        symbol: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<MarketData>, DieselError> {
        market_data::table
            .filter(market_data::asset_symbol.eq(symbol))
            .filter(market_data::timestamp.ge(from))
            .filter(market_data::timestamp.le(to))
            .order(market_data::timestamp.asc())
            .load::<MarketData>(conn)
    }
}