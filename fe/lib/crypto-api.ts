import type { DashboardResponse, HistoricalResponse, ChartDataPoint, CryptoMetric, MacroMetricType } from "./types"

const BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080"

export async function fetchDashboardData(): Promise<DashboardResponse> {
  try {
    const response = await fetch(`${BASE_URL}/api/dashboard`)

    if (!response.ok) {
      throw new Error(`API request failed: ${response.status}`)
    }

    const data = await response.json()
    return data
  } catch (error) {
    console.error("Error fetching dashboard data:", error)
    throw error
  }
}

export async function fetchHistoricalMacroData(symbol: MacroMetricType, days = 64): Promise<ChartDataPoint[]> {
  try {
    const response = await fetch(`${BASE_URL}/api/historical?symbol=${symbol}&days=${days}`)

    if (!response.ok) {
      throw new Error(`API request failed: ${response.status}`)
    }

    const data: HistoricalResponse = await response.json()

    return data.data.map((point) => ({
      timestamp: new Date(point.timestamp).getTime(),
      value: point.value,
    }))
  } catch (error) {
    console.error(`Error fetching historical data for ${symbol}:`, error)
    throw error
  }
}

export function parseAssetData(snapshot: any, metric: CryptoMetric): ChartDataPoint[] {
  return snapshot.prices.map((price: any) => ({
    timestamp: new Date(price.timestamp).getTime(),
    value: metric === "price" ? price.price_usd : metric === "volume" ? price.volume_usd : price.market_cap_usd || 0,
  }))
}