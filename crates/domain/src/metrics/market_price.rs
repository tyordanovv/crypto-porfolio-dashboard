use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone)]
pub struct MarketPrice {
    pub symbol: MarketSymbol,
    pub price_usd: f64,
    pub price_usd_7d_ago: f64,
    pub price_usd_30d_ago: f64,
    pub price_usd_90d_ago: f64,
    pub volume_24h_usd: u64,
}

#[derive(Debug, Clone)]
pub enum MarketSymbol {
    BtcUsd,
    EthUsd,
    Gold,
    Oil,
    Sp500,
    Nasdaq,
    UsdIndex,
    DFF,            // Federal Funds Rate
    T10Y2Y,         // 10Y-2Y Treasury Spread
    DEXUSEU ,       // USD/EUR Exchange Rate
    CPIAUCSL,       // Consumer Price Index
    DGS10 ,         // 10-Year Treasury Constant Maturity Rate
    DGS2,           // 2-Year Treasury Constant Maturity Rate
    M2SL,           // M2 Money Stock
    UNRATE,         // Unemployment Rate
    FEDFUNDS,
    BtcDominance,
    EthDominance,
    StablecoinDominance,
    BtcStableRatio,
    BtcReturn7d,
    BtcReturn30d,
    BtcReturn90d,
    GlobalTotalMarketCapUsd,
    GlobalTotalStableCapUsd,
    GlobalTotalBtcCapUsd,
    GlobalTotalEthCapUsd,
    GlobalTotalVolume24hUsd,
    FearGreedIndex,
}

impl MarketSymbol {
    pub fn as_yahoo_symbol(&self) -> &'static str {
        match self {
            MarketSymbol::BtcUsd => "BTC-USD",
            MarketSymbol::EthUsd => "ETH-USD",
            MarketSymbol::Gold => "GC=F",
            MarketSymbol::Oil => "CL=F",
            MarketSymbol::Sp500 => "^GSPC",
            MarketSymbol::Nasdaq => "^IXIC",
            MarketSymbol::UsdIndex => "DX-Y.NYB",
            _ => "UNKNOWN",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MarketSymbol::Gold => "GOLD_USD",
            MarketSymbol::Oil => "OIL_USD",
            MarketSymbol::Sp500 => "SP500_USD",
            MarketSymbol::Nasdaq => "NASDAQ_USD",
            MarketSymbol::UsdIndex => "USD_INDEX_USD",
            MarketSymbol::DFF => "DFF",
            MarketSymbol::T10Y2Y => "T10Y2Y",
            MarketSymbol::DEXUSEU =>  "DEXUSEU",
            MarketSymbol::CPIAUCSL => "CPIAUCSL",
            MarketSymbol::DGS10 => "DGS10",
            MarketSymbol::DGS2 =>  "DGS2",
            MarketSymbol::M2SL => "M2SL",
            MarketSymbol::UNRATE => "UNRATE",
            MarketSymbol::FEDFUNDS =>  "FEDFUNDS",
            MarketSymbol::BtcDominance => "BTC_DOMINANCE",
            MarketSymbol::BtcUsd => "BTC_USD",
            MarketSymbol::EthUsd => "ETH_USD",
            MarketSymbol::EthDominance => "ETH_DOMINANCE",
            MarketSymbol::StablecoinDominance => "STABLECOIN_DOMINANCE",
            MarketSymbol::BtcStableRatio => "BTC_STABLE_RATIO",
            MarketSymbol::BtcReturn7d => "BTC_RETURN_7D",
            MarketSymbol::BtcReturn30d => "BTC_RETURN_30D",
            MarketSymbol::BtcReturn90d => "BTC_RETURN_90D",
            MarketSymbol::GlobalTotalMarketCapUsd => "GLOBAL_TOTAL_MARKET_CAP_USD",
            MarketSymbol::GlobalTotalStableCapUsd => "GLOBAL_TOTAL_STABLE_CAP_USD",
            MarketSymbol::GlobalTotalBtcCapUsd => "GLOBAL_TOTAL_BTC_CAP_USD",
            MarketSymbol::GlobalTotalEthCapUsd => "GLOBAL_TOTAL_ETH_CAP_USD",
            MarketSymbol::GlobalTotalVolume24hUsd => "GLOBAL_TOTAL_VOLUME_24H_USD",
            MarketSymbol::FearGreedIndex => "FEAR_GREED_INDEX",
        }
    }

    pub fn is_btc(&self) -> bool {
        matches!(self, MarketSymbol::BtcUsd)
    }

    pub fn btc_metrics() -> [MarketSymbol; 5] {
        [
            MarketSymbol::BtcDominance,
            MarketSymbol::BtcStableRatio,
            MarketSymbol::BtcReturn7d,
            MarketSymbol::BtcReturn30d,
            MarketSymbol::BtcReturn90d,
        ]
    }

    pub fn eth_metrics() -> [MarketSymbol; 1] {
        [
            MarketSymbol::EthDominance,
        ]
    }

    pub fn macro_metrics() -> [MarketSymbol; 14] {
        [
            MarketSymbol::Gold,
            MarketSymbol::Oil,
            MarketSymbol::Sp500,
            MarketSymbol::Nasdaq,
            MarketSymbol::UsdIndex,
            MarketSymbol::DFF,
            MarketSymbol::T10Y2Y,
            MarketSymbol::DEXUSEU,
            MarketSymbol::CPIAUCSL,
            MarketSymbol::DGS10,
            MarketSymbol::DGS2,
            MarketSymbol::M2SL,
            MarketSymbol::UNRATE,
            MarketSymbol::FEDFUNDS,
        ]
    }

    pub fn fred_metrics() -> [MarketSymbol; 9] {
        [
            MarketSymbol::DFF,
            MarketSymbol::T10Y2Y,
            MarketSymbol::DEXUSEU,
            MarketSymbol::CPIAUCSL,
            MarketSymbol::DGS10,
            MarketSymbol::DGS2,
            MarketSymbol::M2SL,
            MarketSymbol::UNRATE,
            MarketSymbol::FEDFUNDS,
        ]
    }
}

// Custom error type for parsing
#[derive(Debug, Clone)]
pub struct ParseMarketSymbolError(pub String);

impl fmt::Display for ParseMarketSymbolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid MarketSymbol: {}", self.0)
    }
}

impl std::error::Error for ParseMarketSymbolError {}

impl FromStr for MarketSymbol {
    type Err = ParseMarketSymbolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "BTC_USD" => Ok(MarketSymbol::BtcUsd),
            "ETH_USD" => Ok(MarketSymbol::EthUsd),
            "GOLD_USD" => Ok(MarketSymbol::Gold),
            "OIL_USD" => Ok(MarketSymbol::Oil),
            "SP500_USD" => Ok(MarketSymbol::Sp500),
            "NASDAQ_USD" => Ok(MarketSymbol::Nasdaq),
            "USD_INDEX_USD" => Ok(MarketSymbol::UsdIndex),
            "DFF" => Ok(MarketSymbol::DFF),
            "T10Y2Y" => Ok(MarketSymbol::T10Y2Y),
            "DEXUSEU" => Ok(MarketSymbol::DEXUSEU),
            "CPIAUCSL" => Ok(MarketSymbol::CPIAUCSL),
            "DGS10" => Ok(MarketSymbol::DGS10),
            "DGS2" => Ok(MarketSymbol::DGS2),
            "M2SL" => Ok(MarketSymbol::M2SL),
            "UNRATE" => Ok(MarketSymbol::UNRATE),
            "FEDFUNDS" => Ok(MarketSymbol::FEDFUNDS),
            "BTC_DOMINANCE" => Ok(MarketSymbol::BtcDominance),
            "ETH_DOMINANCE" => Ok(MarketSymbol::EthDominance),
            "STABLECOIN_DOMINANCE" => Ok(MarketSymbol::StablecoinDominance),
            "BTC_STABLE_RATIO" => Ok(MarketSymbol::BtcStableRatio),
            "BTC_RETURN_7D" => Ok(MarketSymbol::BtcReturn7d),
            "BTC_RETURN_30D" => Ok(MarketSymbol::BtcReturn30d),
            "BTC_RETURN_90D" => Ok(MarketSymbol::BtcReturn90d),
            "GLOBAL_TOTAL_MARKET_CAP_USD" => Ok(MarketSymbol::GlobalTotalMarketCapUsd),
            "GLOBAL_TOTAL_STABLE_CAP_USD" => Ok(MarketSymbol::GlobalTotalStableCapUsd),
            "GLOBAL_TOTAL_BTC_CAP_USD" => Ok(MarketSymbol::GlobalTotalBtcCapUsd),
            "GLOBAL_TOTAL_ETH_CAP_USD" => Ok(MarketSymbol::GlobalTotalEthCapUsd),
            "GLOBAL_TOTAL_VOLUME_24H_USD" => Ok(MarketSymbol::GlobalTotalVolume24hUsd),
            "FEAR_GREED_INDEX" => Ok(MarketSymbol::FearGreedIndex),
            other => Err(ParseMarketSymbolError(other.to_string())),
        }
    }
}