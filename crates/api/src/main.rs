mod dtos;
mod handlers;
mod errors;

use std::env;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
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
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .service(btc_dashboard)
            .service(historical_metrics)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}