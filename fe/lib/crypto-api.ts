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
    return generateMockDashboardData()
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
    return generateMockHistoricalData(days)
  }
}

export function parseAssetData(snapshot: any, metric: CryptoMetric): ChartDataPoint[] {
  return snapshot.prices.map((price: any) => ({
    timestamp: new Date(price.timestamp).getTime(),
    value: metric === "price" ? price.price_usd : metric === "volume" ? price.volume_usd : price.market_cap_usd || 0,
  }))
}

// Mock data generators for fallback
function generateMockDashboardData(): DashboardResponse {
  const now = new Date().toISOString().split("T")[0]

  return {
    snapshots: [
      {
        symbol: "BTC_USD",
        prices: Array.from({ length: 30 }, (_, i) => ({
          timestamp: new Date(Date.now() - (29 - i) * 24 * 60 * 60 * 1000).toISOString().split("T")[0],
          price_usd: 45000 + Math.random() * 10000,
          volume_usd: 25000000000 + Math.random() * 10000000000,
          market_cap_usd: 850000000000 + Math.random() * 50000000000,
          dominance: null,
        })),
        metrics: [
          { timestamp: now, value: 0.59, name: "BTC_DOMINANCE", formatedName: "BTC Dominance" },
          { timestamp: now, value: -0.025, name: "BTC_RETURN_30D", formatedName: "BTC 30 days return" },
          { timestamp: now, value: 0.038, name: "BTC_RETURN_7D", formatedName: "BTC 7 days return" },
          { timestamp: now, value: -0.063, name: "BTC_RETURN_90D", formatedName: "BTC 90 days return" },
          { timestamp: now, value: 7.81, name: "BTC_STABLE_RATIO", formatedName: "BTC to stablecoin ratio" },
        ],
      },
      {
        symbol: "ETH_USD",
        prices: Array.from({ length: 30 }, (_, i) => ({
          timestamp: new Date(Date.now() - (29 - i) * 24 * 60 * 60 * 1000).toISOString().split("T")[0],
          price_usd: 2500 + Math.random() * 1000,
          volume_usd: 15000000000 + Math.random() * 5000000000,
          market_cap_usd: 300000000000 + Math.random() * 20000000000,
          dominance: null,
        })),
        metrics: [{ timestamp: now, value: 0.127, name: "ETH_DOMINANCE", formatedName: "ETH Dominance" }],
      },
    ],
    macro_metrics: [
      { name: "CPIAUCSL", value: 323.364, source: "fred", formattedName: "Consumer Price Index (CPI)" },
      { name: "DEXUSEU", value: 1.1674, source: "fred", formattedName: "USD to Euro Exchange Rate" },
      { name: "DFF", value: 4.11, source: "fred", formattedName: "Federal Funds Effective Rate" },
      { name: "DGS10", value: 3.98, source: "fred", formattedName: "10-Year Treasury Yield" },
      { name: "DGS2", value: 3.45, source: "fred", formattedName: "2-Year Treasury Yield" },
      { name: "FEDFUNDS", value: 4.22, source: "fred", formattedName: "Federal Funds Rate (Target)" },
      { name: "M2SL", value: 22195.4, source: "fred", formattedName: "M2 Money Supply (Billions USD)" },
      { name: "T10Y2Y", value: 0.52, source: "fred", formattedName: "10Y-2Y Treasury Yield Spread" },
      { name: "UNRATE", value: 4.3, source: "fred", formattedName: "Unemployment Rate (%)" }
    ],
  }
}

function generateMockHistoricalData(days: number): ChartDataPoint[] {
  const now = Date.now()
  const data: ChartDataPoint[] = []

  for (let i = days; i >= 0; i--) {
    const timestamp = now - i * 24 * 60 * 60 * 1000
    let value: number

    if (i > 40) {
      const progress = (days - i) / (days - 40)
      value = 20 + progress * 30 + (Math.random() - 0.5) * 8
    } else if (i > 20) {
      const progress = (40 - i) / 20
      value = 50 + progress * 25 + (Math.random() - 0.5) * 6
    } else if (i > 10) {
      value = 75 + Math.random() * 5 + (Math.random() - 0.5) * 4
    } else {
      const progress = (10 - i) / 10
      value = 80 - progress * 25 + (Math.random() - 0.5) * 5
    }

    data.push({
      timestamp,
      value: Math.max(0, Math.min(100, value)),
    })
  }

  return data
}
