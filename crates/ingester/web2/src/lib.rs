pub mod fetchers;
pub mod models;
pub mod client;

pub use fetchers::{MacroDataFetcher, MarketDataFetcher};
pub use models::{FearGreedIndex, FredIndicator, CryptoPrice, GlobalMarketData};
pub use client::Web2Client;