use std::str::FromStr;

use actix_web::{get, HttpResponse};
use domain::MarketSymbol;
use serde::Deserialize;
use store::{db::PgPool, repositories::{market_data_repository::MarketDataRepo, market_metrics_repository::MarketMetricRepo}};
use crate::dtos::*;
use actix_web::{web, Result};

#[get("/api/dashboard")]
async fn btc_dashboard(db_pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let mut conn = db_pool.get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let latest_btc_values = MarketDataRepo::latest_n_for_asset(&mut conn, MarketSymbol::BtcUsd, 365).await.unwrap();
    let latest_btc_metrics = MarketMetricRepo::latest_array_metrics(&mut conn, &MarketSymbol::btc_metrics()).await.unwrap();

    let latest_eth_values = MarketDataRepo::latest_n_for_asset(&mut conn, MarketSymbol::EthUsd, 365).await.unwrap();
    let latest_eth_metrics = MarketMetricRepo::latest_array_metrics(&mut conn, &MarketSymbol::eth_metrics()).await.unwrap();

    let latest_macro_metrics = MarketMetricRepo::latest_array_metrics(&mut conn, &MarketSymbol::macro_metrics()).await.unwrap();
    let latest_fg = MarketMetricRepo::latest_n(&mut conn, MarketSymbol::FearGreedIndex, 378).await.unwrap();

    let response = DashboardResponse { 
        snapshots: vec![
            AssetSnapshot::from_market_data(MarketSymbol::BtcUsd, latest_btc_values, latest_btc_metrics), 
            AssetSnapshot::from_market_data(MarketSymbol::EthUsd, latest_eth_values, latest_eth_metrics)
        ],
        fear_greed: FearGreedIndex::from_market_data(latest_fg),
        macro_metrics: MacroMetrics::from_market_data(latest_macro_metrics), 
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/api/historical")]
async fn historical_metrics(
    db_pool: web::Data<PgPool>,
    query: web::Query<HistoricalMetricsQuery>,
) -> Result<HttpResponse> {
    let mut conn = db_pool.get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let symbol = MarketSymbol::from_str(query.symbol.as_str()).unwrap();
    let days = query.days;

    let data = MarketMetricRepo::latest_n(&mut conn, symbol, days)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(MacroMetrics::from_market_data(data)))
}

#[derive(Deserialize)]
pub struct HistoricalMetricsQuery {
    pub symbol: String,
    pub days: i64,
}