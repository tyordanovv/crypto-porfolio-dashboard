use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::dsl::insert_into;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use crate::db::PgPooledConnection;
use crate::models::{ Indicator, NewIndicator};
use crate::schema::indicators;

/// Indicator repository
pub struct IndicatorRepo;

impl IndicatorRepo {
    pub fn insert(conn: &mut PgPooledConnection, rec: &NewIndicator) -> Result<usize, DieselError> {
        insert_into(indicators::table)
            .values(rec)
            .on_conflict((indicators::name, indicators::timestamp))
            .do_update()
            .set(indicators::value.eq(excluded(indicators::value)))
            .execute(conn)
    }

    pub fn latest(conn: &mut PgPooledConnection, name: &str) -> Result<Option<Indicator>, DieselError> {
        indicators::table
            .filter(indicators::name.eq(name))
            .order(indicators::timestamp.desc())
            .first::<Indicator>(conn)
            .optional()
    }

    pub fn range(
        conn: &mut PgPooledConnection,
        name: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<Indicator>, DieselError> {
        indicators::table
            .filter(indicators::name.eq(name))
            .filter(indicators::timestamp.ge(from))
            .filter(indicators::timestamp.le(to))
            .order(indicators::timestamp.asc())
            .load::<Indicator>(conn)
    }
}