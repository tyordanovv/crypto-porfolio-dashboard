use anyhow::{Context, Result};
use std::collections::HashMap;
use crate::client::Web2Client;
use crate::models::{
    CryptoPrice, GlobalMarketData, 
    CoinGeckoSimplePrice, CoinGeckoGlobal
};

pub struct MarketDataFetcher<'a> {
    client: &'a Web2Client,
}

impl<'a> MarketDataFetcher<'a> {
    pub fn new(client: &'a Web2Client) -> Self {
        Self { client }
    }

    pub async fn fetch_crypto_price(
        &self,
        coin_id: &str,
        symbol: &str,
    ) -> Result<CryptoPrice> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price\
             ?ids={}&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true",
            coin_id
        );

        let mut response: HashMap<String, CoinGeckoSimplePrice> = self.client
            .http()
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch {} price", coin_id))?
            .json()
            .await
            .context("Failed to parse CoinGecko response")?;

        let data = response
            .remove(coin_id)
            .context(format!("No data for {}", coin_id))?;

        Ok(CryptoPrice {
            symbol: symbol.to_string(),
            price_usd: data.usd,
            market_cap_usd: data.usd_market_cap,
            volume_24h_usd: data.usd_24h_vol,
        })
    }

    pub async fn fetch_global_market_data(&self) -> Result<GlobalMarketData> {
        let url = "https://api.coingecko.com/api/v3/global";

        let response: CoinGeckoGlobal = self.client
            .http()
            .get(url)
            .send()
            .await
            .context("Failed to fetch global market data")?
            .json()
            .await
            .context("Failed to parse global market data")?;

        let total_market_cap = response
            .data
            .total_market_cap
            .get("usd")
            .copied()
            .context("USD market cap not found")?;

        let total_volume = response
            .data
            .total_volume
            .get("usd")
            .copied()
            .context("USD volume not found")?;

        let btc_dominance = response
            .data
            .market_cap_percentage
            .get("btc")
            .copied()
            .context("BTC dominance not found")?;

        let eth_dominance = response
            .data
            .market_cap_percentage
            .get("eth")
            .copied()
            .context("BTC dominance not found")?;

        Ok(GlobalMarketData {
            total_market_cap_usd: total_market_cap,
            total_volume_24h_usd: total_volume,
            btc_dominance,
            eth_dominance,
        })
    }

    pub async fn fetch_multiple_prices(
        &self,
        coins: &[(&str, &str)], // (coin_id, symbol)
    ) -> Vec<(String, Result<CryptoPrice>)> {
        let futures = coins
            .iter()
            .map(|(coin_id, symbol)| async move {
                let result = self.fetch_crypto_price(coin_id, symbol).await;
                (symbol.to_string(), result)
            });

        futures::future::join_all(futures).await
    }
}