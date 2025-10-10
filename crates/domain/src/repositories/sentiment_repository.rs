use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::dsl::insert_into;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use crate::db::PgPooledConnection;
use crate::models::{ SentimentData, NewSentimentData };
use crate::schema::sentiment_data;

/// Sentiment repository
pub struct SentimentRepo;

impl SentimentRepo {
    pub fn insert(conn: &mut PgPooledConnection, rec: &NewSentimentData) -> Result<usize, DieselError> {
        insert_into(sentiment_data::table)
            .values(rec)
            .on_conflict((sentiment_data::name, sentiment_data::timestamp))
            .do_update()
            .set(sentiment_data::value.eq(excluded(sentiment_data::value)))
            .execute(conn)
    }

    pub fn latest(conn: &mut PgPooledConnection, name: &str) -> Result<Option<SentimentData>, DieselError> {
        sentiment_data::table
            .filter(sentiment_data::name.eq(name))
            .order(sentiment_data::timestamp.desc())
            .first::<SentimentData>(conn)
            .optional()
    }

    pub fn range(
        conn: &mut PgPooledConnection,
        name: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<SentimentData>, DieselError> {
        sentiment_data::table
            .filter(sentiment_data::name.eq(name))
            .filter(sentiment_data::timestamp.ge(from))
            .filter(sentiment_data::timestamp.le(to))
            .order(sentiment_data::timestamp.asc())
            .load::<SentimentData>(conn)
    }
}
