use actix_web::{get, HttpResponse};
use domain::MarketSymbol;
use store::{db::PgPool, repositories::{market_data_repository::MarketDataRepo, market_metrics_repository::MarketMetricRepo}};
use crate::dtos::*;
use actix_web::{web, Result};

#[get("/api/btc/dashboard")]
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