#[cfg(test)]
pub mod indicators_tests;
#[cfg(test)]
pub mod market_data_tests;
#[cfg(test)]
pub mod sentiment_tests;
#[cfg(test)]
pub mod signal_tests;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_test_pool() -> PgPool {
    dotenvy::from_filename(".env.test").ok();


    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set for tests");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.")
}