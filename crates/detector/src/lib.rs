use dex_core::{PoolPrice, SpreadEvent};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn detect_spread(prices: &[PoolPrice]) -> Option<SpreadEvent> {
    if prices.len() < 2 { return None; }

    let mut max_price = prices[0];
    let mut min_price = prices[0];

    for &p in prices.iter() {
        if p.price > max_price.price { max_price = p; }
        if p.price < min_price.price { min_price = p; }
    }

    let spread = (max_price.price - min_price.price) / min_price.price * 100.0;
    if spread > 0.3 {
        Some(SpreadEvent {
            pair: (min_price.token_in, min_price.token_out),
            dex_a: min_price.dex_id,
            dex_b: max_price.dex_id,
            spread_pct: spread,
            ts: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        })
    } else {
        None
    }
}
