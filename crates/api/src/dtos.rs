use std::{collections::HashSet, str::FromStr};

use domain::MarketSymbol;
use serde::Serialize;
use chrono::NaiveDate;
use store::models::{market_data_db::MarketDataDB, market_metrics_db::MarketMetricDataDB};

#[derive(Serialize)]
pub struct DashboardResponse {
    pub snapshots: Vec<AssetSnapshot>,
    pub fear_greed: FearGreedIndex,
    pub macro_metrics: Option<MacroMetrics>,
}

#[derive(Serialize)]
pub struct AssetSnapshot {
    pub symbol: String,
    #[serde(rename = "formattedName")]
    pub formatted_name: String,
    pub prices: PriceData,
    pub metrics: Vec<MetricData>,
}

impl AssetSnapshot {
    pub fn from_market_data(symbol: MarketSymbol, market_data: Vec<MarketDataDB>, market_metric: Vec<MarketMetricDataDB>) -> Self {
        Self {
            symbol: symbol.as_str().to_string(),
            formatted_name: symbol.formatted_name().to_string(),
            prices: PriceData::from_market_data(market_data),
            metrics: Self::group_metrics(market_metric),
        }
    }

    fn group_metrics(metrics: Vec<MarketMetricDataDB>) -> Vec<MetricData> {
        let mut grouped: std::collections::HashMap<String, Vec<MarketMetricDataDB>> = std::collections::HashMap::new();
        
        for metric in metrics {
            grouped.entry(metric.name.clone()).or_insert_with(Vec::new).push(metric);
        }
        
        grouped.into_iter().map(|(name, data)| {
            MetricData::from_market_metrics(name, data)
        }).collect()
    }
}

// Compact array format: [timestamp, price, volume]
#[derive(Serialize)]
pub struct PriceData {
    pub data: Vec<(i64, f64, Option<f64>)>,
}

impl PriceData {
    pub fn from_market_data(market_data: Vec<MarketDataDB>) -> Self {
        Self {
            data: market_data
                .into_iter()
                .map(|md| {
                    let timestamp = md.timestamp.and_hms_opt(0, 0, 0)
                        .unwrap()
                        .and_utc()
                        .timestamp_millis();
                    (timestamp, md.price_usd, md.volume_usd)
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
pub struct MetricData {
    pub name: String,
    #[serde(rename = "formattedName")]
    pub formatted_name: String,
    // Array of [timestamp, value]
    pub data: Vec<(i64, f64)>,
}

impl MetricData {
    pub fn from_market_metrics(name: String, metrics: Vec<MarketMetricDataDB>) -> Self {
        let formatted_name = match MarketSymbol::from_str(&name) {
            Ok(symbol) => symbol.formatted_name().to_string(),
            Err(_) => name.clone(),
        };

        let data = metrics
            .into_iter()
            .map(|mm| {
                let timestamp = mm.timestamp.and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp_millis();
                (timestamp, mm.value.unwrap_or(0.0))
            })
            .collect();

        Self {
            name,
            formatted_name,
            data,
        }
    }
}

// Compact array format: [timestamp, value, avg7d, avg14d, avg21d, classification]
#[derive(Serialize)]
pub struct FearGreedIndex {
    pub data: Vec<(i64, i32, i32, i32, i32)>,
}

impl FearGreedIndex {
    pub fn from_market_data(entries: Vec<MarketMetricDataDB>) -> Self {
        let mut data = entries;
        data.sort_by_key(|d| d.timestamp);

        let results: Vec<(i64, i32, i32, i32, i32)> = (0..data.len())
            .map(|i| {
                let get_avg = |window: usize| -> i32 {
                    let start = if i + 1 >= window { i + 1 - window } else { 0 };
                    let slice = &data[start..=i];
                    let sum: f64 = slice.iter().filter_map(|d| d.value).sum();
                    let count = slice.iter().filter(|d| d.value.is_some()).count().max(1);
                    (sum / count as f64).round() as i32
                };

                let timestamp = data[i].timestamp.and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp_millis();

                (
                    timestamp,
                    data[i].value.map(|v| v.round() as i32).unwrap_or(0),
                    get_avg(7),
                    get_avg(14),
                    get_avg(21)
                )
            })
            .collect();

        Self { data: results }
    }
}

// Compact array format for macro metrics
#[derive(Serialize)]
pub struct MacroMetrics {
    pub data: Vec<MacroMetricEntry>,
}

#[derive(Serialize)]
pub struct MacroMetricEntry {
    pub name: String,
    #[serde(rename = "formattedName")]
    pub formatted_name: String,
    // Array of [timestamp, value, source]
    pub values: Vec<(NaiveDate, f64, Option<String>)>,
}

impl MacroMetrics {
    pub fn from_market_data(entries: Vec<MarketMetricDataDB>) -> Option<Self> {
        let allowed: HashSet<String> = MarketSymbol::macro_metrics()
            .iter()
            .map(|symbol| symbol.as_str().to_owned())
            .collect();

        let mut grouped: std::collections::HashMap<String, Vec<MarketMetricDataDB>> = std::collections::HashMap::new();
        
        for entry in entries {
            if allowed.contains(&entry.name) && entry.value.is_some() {
                grouped.entry(entry.name.clone()).or_insert_with(Vec::new).push(entry);
            }
        }

        if grouped.is_empty() {
            return None;
        }

        let data: Vec<MacroMetricEntry> = grouped
            .into_iter()
            .filter_map(|(name, metrics)| {
                MarketSymbol::from_str(&name).ok().map(|symbol| {
                    let values = metrics
                        .into_iter()
                        .filter_map(|m| {
                            m.value.map(|v| (m.timestamp, v, m.source.clone()))
                        })
                        .collect();

                    MacroMetricEntry {
                        name: name.clone(),
                        formatted_name: symbol.formatted_name().to_string(),
                        values,
                    }
                })
            })
            .collect();

        if data.is_empty() { None } else { Some(Self { data }) }
    }
}