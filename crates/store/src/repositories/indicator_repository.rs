use chrono::NaiveDate;
use diesel::query_dsl::methods::{FilterDsl, OrderDsl};
use diesel::upsert::excluded;
use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, insert_into};
use diesel::result::Error as DieselError;

use crate::{db::PgPooledConnection, models::indicator_db::IndicatorDB, schema::indicators};

/// Indicator repository
pub struct IndicatorRepo;

impl IndicatorRepo {
    pub fn insert(conn: &mut PgPooledConnection, rec: &IndicatorDB) -> Result<usize, DieselError> {
        insert_into(indicators::table)
            .values(rec)
            .on_conflict((indicators::name, indicators::timestamp))
            .do_update()
            .set(indicators::value.eq(excluded(indicators::value)))
            .execute(conn)
    }

    pub fn latest(conn: &mut PgPooledConnection, name: &str) -> Result<Option<IndicatorDB>, DieselError> {
        indicators::table
            .filter(indicators::name.eq(name))
            .order(indicators::timestamp.desc())
            .first::<IndicatorDB>(conn)
            .optional()
    }

    pub fn range(
        conn: &mut PgPooledConnection,
        name: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<IndicatorDB>, DieselError> {
        indicators::table
            .filter(indicators::name.eq(name))
            .filter(indicators::timestamp.ge(from))
            .filter(indicators::timestamp.le(to))
            .order(indicators::timestamp.asc())
            .load::<IndicatorDB>(conn)
    }
}