use crate::PoolPrice;

#[async_trait::async_trait]
pub trait DexAdapter: Send + Sync {
    async fn poll_prices(&self) -> Vec<PoolPrice>;
}