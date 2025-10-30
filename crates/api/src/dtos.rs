use std::{collections::HashSet, str::FromStr};

use domain::MarketSymbol;
use serde::Serialize;
use chrono::NaiveDate;
use store::models::{market_data_db::MarketDataDB, market_metrics_db::MarketMetricDataDB};

#[derive(Serialize)]
pub struct DashboardResponse {
    pub snapshots: Vec<AssetSnapshot>,
    pub fear_greed: Vec<FearGreedIndex>,
    pub macro_metrics: Option<Vec<MacroMetrics>>,
}

#[derive(Serialize)]
pub struct AssetSnapshot {
    pub symbol: String,
    #[serde(rename = "formattedName")]
    pub formatted_name: String,
    pub prices: Vec<TimeSeriesPrice>,
    pub metrics: Vec<MarketMetric>
}

impl AssetSnapshot {
    pub fn from_market_data(symbol: MarketSymbol, market_data: Vec<MarketDataDB>, market_metric: Vec<MarketMetricDataDB>) -> Self {
        Self {
            symbol: symbol.as_str().to_string(),
            formatted_name: symbol.formatted_name().to_string(),
            prices: market_data
                .into_iter()
                .map(TimeSeriesPrice::from)
                .collect(),
            metrics: market_metric
                .into_iter()
                .map(MarketMetric::from)
                .collect()
        }
    }
}

#[derive(Serialize)]
pub struct TimeSeriesPrice { 
    pub timestamp: NaiveDate, 
    pub price_usd: f64, 
    pub volume_usd: Option<f64>
}

impl From<MarketDataDB> for TimeSeriesPrice {
    fn from(md: MarketDataDB) -> Self {
        Self {
            timestamp: md.timestamp,
            price_usd: md.price_usd,
            volume_usd: md.volume_usd
        }
    }
}

#[derive(Serialize)]
pub struct MarketMetric {
    pub timestamp: NaiveDate, 
    pub value: f64, 
    pub name: String,
    #[serde(rename = "formattedName")]
    pub formatted_name: String
}

impl From<MarketMetricDataDB> for MarketMetric {
    fn from(mm: MarketMetricDataDB) -> Self {
        let formatted_name = match MarketSymbol::from_str(&mm.name) {
            Ok(symbol) => symbol.formatted_name().to_string(),
            Err(_) => mm.name.clone(), // fallback if unknown
        };

        Self {
            timestamp: mm.timestamp,
            value: mm.value.unwrap_or(0.0),
            name: mm.name,
            formatted_name,
        }
    }
}

#[derive(Serialize)]
pub struct FearGreedIndex {
    pub timestamp: NaiveDate,
    pub value: i32,
    pub value_avg_7d: i32,
    pub value_avg_14d: i32,
    pub value_avg_21d: i32,
    pub classification: String,
}

impl FearGreedIndex {
    pub fn from_market_data(entries: Vec<MarketMetricDataDB>) -> Vec<Self> {
        let mut data = entries;
        data.sort_by_key(|d| d.timestamp);

        let mut results = Vec::with_capacity(data.len());

        for i in 0..data.len() {
            let get_avg = |window: usize| -> i32 {
                let start = if i + 1 >= window { i + 1 - window } else { 0 };
                let slice = &data[start..=i];
                let sum: f64 = slice
                    .iter()
                    .filter_map(|d| d.value)
                    .sum();
                let count = slice.iter().filter(|d| d.value.is_some()).count().max(1);
                (sum / count as f64).round() as i32
            };

            results.push(FearGreedIndex {
                timestamp: data[i].timestamp,
                value: data[i].value.map(|v| v.round() as i32).unwrap_or(0),
                value_avg_7d: get_avg(7),
                value_avg_14d: get_avg(14),
                value_avg_21d: get_avg(21),
                classification: data[i].source.clone().unwrap(),
            });
        }

        results
    }
}

#[derive(Serialize)]
pub struct MacroMetrics {
    pub name: String,
    #[serde(rename = "formattedName")]
    pub formatted_name: String,
    pub timestamp: NaiveDate,
    pub value: f64,
    pub source: Option<String>,
}

impl MacroMetrics {
    pub fn from_market_data(entries: Vec<MarketMetricDataDB>) -> Option<Vec<Self>> {
        let allowed: HashSet<String> = MarketSymbol::macro_metrics()
            .iter()
            .map(|symbol| symbol.as_str().to_owned())
            .collect();

        let macro_metrics: Vec<MacroMetrics> = entries
            .into_iter()
            .filter(|entry| allowed.contains(&entry.name)) // only keep known metrics
            .filter_map(|entry| {
                // only proceed if we have a value
                entry.value.map(|v| {
                    match MarketSymbol::from_str(&entry.name) {
                        Ok(symbol) => Some(MacroMetrics {
                            name: entry.name.clone(),
                            formatted_name: symbol.formatted_name().to_string(),
                            timestamp: entry.timestamp,
                            value: v,
                            source: entry.source.clone()
                        }),
                        Err(_) => None,
                    }
                })
                .flatten()
            })
            .collect();
        
        if macro_metrics.is_empty() { None } else { Some(macro_metrics) }
    }
}