use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PoolPrice {
    pub dex_id: u16,
    pub pool_id: u64,
    pub token_in: u32,
    pub token_out: u32,
    pub price: f64,
    pub liquidity: f64,
    pub timestamp: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadEvent {
    pub pair: (u32, u32),
    pub dex_a: u16,
    pub dex_b: u16,
    pub spread_pct: f64,
    pub ts: u128,
}