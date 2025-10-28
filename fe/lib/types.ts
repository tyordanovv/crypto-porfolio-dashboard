export interface PriceData {
  timestamp: string
  price_usd: number
  volume_usd: number
  market_cap_usd: number | null
  dominance: number | null
}

export interface MetricData {
  timestamp: string
  value: number
  name: string
  formatedName: string
}

export interface AssetSnapshot {
  symbol: string
  prices: PriceData[]
  metrics: MetricData[]
}

export interface MacroMetricData {
  name: string
  value: number
  source: string
  formattedName: string
}

export interface DashboardResponse {
  snapshots: AssetSnapshot[]
  macro_metrics: MacroMetricData[]
}

export interface HistoricalDataPoint {
  timestamp: string
  value: number
}

export interface HistoricalResponse {
  data: HistoricalDataPoint[]
}

// Chart Types
export type CryptoMetric = "price" | "volume" | "market_cap"
export type MacroMetricType =
  | "fear_and_greed"
  | "CPIAUCSL"
  | "DEXUSEU"
  | "DFF"
  | "DGS10"
  | "DGS2"
  | "FEDFUNDS"
  | "M2SL"
  | "T10Y2Y"
  | "UNRATE"

export interface ChartDataPoint {
  timestamp: number
  [key: string]: number
}
