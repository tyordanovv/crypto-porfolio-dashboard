use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::dsl::insert_into;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use domain::MarketSymbol;
use crate::db::PgPooledConnection;
use crate::models::market_metrics_db::MarketMetricDataDB;
use crate::schema::market_metrics;

/// Sentiment repository
pub struct MarketMetricRepo;

impl MarketMetricRepo {
    pub async fn insert(conn: &mut PgPooledConnection, rec: &MarketMetricDataDB) -> Result<usize, DieselError> {
        insert_into(market_metrics::table)
            .values(rec)
            .on_conflict((market_metrics::name, market_metrics::timestamp))
            .do_update()
            .set(market_metrics::value.eq(excluded(market_metrics::value)))
            .execute(conn)
    }

    pub async fn latest_n(
        conn: &mut PgPooledConnection,
        symbol: MarketSymbol,
        limit: i64,
    ) -> Result<Vec<MarketMetricDataDB>, DieselError> {
        market_metrics::table
            .filter(market_metrics::name.eq(symbol.as_str()))
            .order(market_metrics::timestamp.desc())
            .limit(limit)
            .load::<MarketMetricDataDB>(conn)
    }

    pub async fn latest_array_metrics(
        conn: &mut PgPooledConnection,
        metric_symbols: &[MarketSymbol],
    ) -> Result<Vec<MarketMetricDataDB>, DieselError> {
        use crate::schema::market_metrics::dsl::*;

        let names: Vec<&str> = metric_symbols.iter().map(|s| s.as_str()).collect();

        let mut rows = market_metrics
            .filter(name.eq_any(&names))
            .order((name.asc(), timestamp.desc()))
            .load::<MarketMetricDataDB>(conn)?;

        rows.sort_by(|a, b| a.name.cmp(&b.name));
        rows.dedup_by(|a, b| a.name == b.name);

        Ok(rows)
    }

    pub async fn range(
        conn: &mut PgPooledConnection,
        name: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<MarketMetricDataDB>, DieselError> {
        market_metrics::table
            .filter(market_metrics::name.eq(name))
            .filter(market_metrics::timestamp.ge(from))
            .filter(market_metrics::timestamp.le(to))
            .order(market_metrics::timestamp.asc())
            .load::<MarketMetricDataDB>(conn)
    }
}
