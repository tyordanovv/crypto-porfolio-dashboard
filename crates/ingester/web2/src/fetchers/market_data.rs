use anyhow::{Context, Result};
use std::collections::HashMap;
use crate::client::Web2Client;
use crate::models::{
    CoinGeckoGlobal, CoinGeckoSimplePrice, CoinMarketCapData, CoinMarketCapPoint, CoinMarketCapResponseData, CryptoPrice, GlobalMarketData
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
        let url = "https://api.coinmarketcap.com/data-api/v4/global-metrics/quotes/historical?convertId=2781&range=30d";

        // let response: CoinGeckoGlobal = self.client
        //     .http()
        //     .get(url)
        //     .send()
        //     .await
        //     .context("Failed to fetch global market data")?
        //     .json()
        //     .await
        //     .context("Failed to parse global market data")?;

        let response: CoinMarketCapResponseData = self.client
            .http()
            .get(url)
            .send()
            .await
            .context("Failed to fetch global market data")?
            .json()
            .await
            .context("Failed to parse global market data")?;
        
        let target_ts = response.data.yearlyPerformance.high.timestamp.parse::<i64>().ok().unwrap();

        let last_full_day = response.data.points.iter().find(|p| {
            if let Ok(ts) = p.timestamp.parse::<u64>() {
                (ts as i64 - target_ts as i64).abs() < 3600
            } else {
                false
            }
        }).unwrap();
        

        println!("✅ Found latest full day:");
        println!("MarketCap: {}", last_full_day.marketCap);
        println!("StableValue: {}", last_full_day.stableValue);
        println!("Volume: {}", last_full_day.volume);

        Ok(GlobalMarketData {
            total_market_cap_usd: last_full_day.marketCap,
            total_stable_cap_usd: last_full_day.stableValue,
            total_volume_24h_usd: last_full_day.volume
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

    // fn get_latest_full_day(&self, data: &CoinMarketCapData) -> Option<&CoinMarketCapPoint> {
    //     let target_ts = data.yearlyPerformance.high.timestamp.parse::<u64>().ok()?;

    //     data.points.iter().find(|p| {
    //         if let Ok(ts) = p.timestamp.parse::<u64>() {
    //             (ts as i64 - target_ts as i64).abs() < 3600 // within 1 hour tolerance
    //         } else {
    //             false
    //         }
    //     })
    // }
}