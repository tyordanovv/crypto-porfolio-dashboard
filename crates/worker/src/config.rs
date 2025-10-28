use domain::MarketSymbol;
use web2::clients::M2Country;

#[derive(Debug, Clone)]
pub struct DailyWorkerConfig {
    pub fred_series: Vec<MarketSymbol>,
    pub crypto_pairs: Vec<MarketSymbol>,
    pub market_series: Vec<MarketSymbol>,
}

impl Default for DailyWorkerConfig {
    fn default() -> Self {
        Self {
            fred_series: vec![
                MarketSymbol::DFF,      // Federal Funds Rate
                MarketSymbol::T10Y2Y,   // 10Y-2Y Treasury Spread
                MarketSymbol::DEXUSEU,  // USD/EUR Exchange Rate
                MarketSymbol::CPIAUCSL, // Consumer Price Index
                MarketSymbol::DGS10,    // 10-Year Treasury Constant Maturity Rate
                MarketSymbol::DGS2,     // 2-Year Treasury Constant Maturity Rate
                MarketSymbol::M2SL,     // M2 Money Stock
                MarketSymbol::UNRATE,   // Unemployment Rate
                MarketSymbol::FEDFUNDS, // Effective Federal Funds Rate
            ],
            crypto_pairs: vec![
                MarketSymbol::BtcUsd,
                MarketSymbol::EthUsd,
            ],
            market_series: vec![
                MarketSymbol::Gold,
                MarketSymbol::Oil,
                MarketSymbol::Sp500,
                MarketSymbol::Nasdaq,
                MarketSymbol::UsdIndex,

            ]
        }
    }
}

#[derive(Debug, Clone)]
pub struct MontlyWorkerConfig {
    pub m2_countries: Vec<M2Country>,
}

impl Default for MontlyWorkerConfig {
    fn default() -> Self {
        Self {
            m2_countries: vec![
                M2Country::US,
                M2Country::EU,
                M2Country::UK,
                M2Country::Japan,
                M2Country::Canada,
                M2Country::China
            ]
        }
    }
}
