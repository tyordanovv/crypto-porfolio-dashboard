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
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MarketSymbol::BtcUsd => "BTC",
            MarketSymbol::EthUsd => "ETH",
            MarketSymbol::Gold => "GOLD",
            MarketSymbol::Oil => "OIL",
            MarketSymbol::Sp500 => "SP500",
            MarketSymbol::Nasdaq => "NASDAQ",
            MarketSymbol::UsdIndex => "USD_INDEX",
        }
    }

    pub fn is_btc(&self) -> bool {
        matches!(self, MarketSymbol::BtcUsd)
    }
}