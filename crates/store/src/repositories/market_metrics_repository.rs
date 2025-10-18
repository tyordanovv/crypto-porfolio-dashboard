use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::dsl::insert_into;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use crate::db::PgPooledConnection;
use crate::models::market_metrics_db::SentimentDataDB;
use crate::schema::market_metrics;

/// Sentiment repository
pub struct SentimentRepo;

impl SentimentRepo {
    pub fn insert(conn: &mut PgPooledConnection, rec: &SentimentDataDB) -> Result<usize, DieselError> {
        insert_into(market_metrics::table)
            .values(rec)
            .on_conflict((market_metrics::name, market_metrics::timestamp))
            .do_update()
            .set(market_metrics::value.eq(excluded(market_metrics::value)))
            .execute(conn)
    }

    pub fn latest(conn: &mut PgPooledConnection, name: &str) -> Result<Option<SentimentDataDB>, DieselError> {
        market_metrics::table
            .filter(market_metrics::name.eq(name))
            .order(market_metrics::timestamp.desc())
            .first::<SentimentDataDB>(conn)
            .optional()
    }

    pub fn range(
        conn: &mut PgPooledConnection,
        name: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<SentimentDataDB>, DieselError> {
        market_metrics::table
            .filter(market_metrics::name.eq(name))
            .filter(market_metrics::timestamp.ge(from))
            .filter(market_metrics::timestamp.le(to))
            .order(market_metrics::timestamp.asc())
            .load::<SentimentDataDB>(conn)
    }
}
