pub mod fetchers;
pub mod models;
pub mod clients;

pub use fetchers::{MacroDataFetcher, MarketDataFetcher};
pub use models::{FearGreedIndex, FredIndicator, CryptoPrice, GlobalCryptoMarketData, GlobalM2Data};