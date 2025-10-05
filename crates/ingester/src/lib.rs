use async_trait::async_trait;
use dex_core::{DexAdapter, PoolPrice};

pub struct CetusIngester {
    pub rpc_url: String,
}

#[async_trait]
impl DexAdapter for CetusIngester {
    async fn poll_prices(&self) -> Vec<PoolPrice> {
        vec![PoolPrice {
            dex_id: 1,
            pool_id: 1,
            token_in: 1,
            token_out: 2,
            price: 1.02,
            liquidity: 100000.0,
            timestamp: 0,
        }]
    }
}