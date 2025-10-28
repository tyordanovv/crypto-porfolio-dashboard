mod dtos;
mod handlers;

use std::env;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use store::db::establish_pool;
use telemetry::setup_observability;

use crate::handlers::{btc_dashboard, historical_metrics};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  
    setup_observability();
    let port: u16 = env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16 number");

    let db_pool = establish_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(btc_dashboard)
            .service(historical_metrics)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}