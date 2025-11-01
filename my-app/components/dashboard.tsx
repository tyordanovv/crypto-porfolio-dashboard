"use client"

import { useEffect, useState } from "react"
import { PriceCard } from "@/components/price-card"
import { FearGreedGauge } from "@/components/fear-greed-gauge"
import { MacroMetricsGrid } from "@/components/macro-metrics-grid"
import { DualAxisChart } from "@/components/dual-axis-chart"
import { Loader2 } from "lucide-react"
import { fetchDashboardData } from "@/lib/mock-data"

interface DashboardData {
  snapshots: Array<{
    symbol: string
    formattedName: string
    prices: {
      data: Array<[number, number, number]>
    }
    metrics: Array<{
      name: string
      formattedName: string
      data: Array<[number, number]>
    }>
  }>
  fear_greed: {
    data: Array<[number, number, number, number, number, string]>
  }
  macro_metrics: {
    data: Array<{
      name: string
      formattedName: string
      values: Array<[string, number, string]>
    }>
  }
}

export function Dashboard() {
  const [data, setData] = useState<DashboardData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [selectedMetric, setSelectedMetric] = useState<{
    name: string
    formattedName: string
  } | null>(null)

  useEffect(() => {
    async function fetchData() {
      try {
        const result = await fetchDashboardData()
        setData(result)
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred")
      } finally {
        setLoading(false)
      }
    }

    fetchData()
  }, [])

  useEffect(() => {
    if (data && !selectedMetric) {
      const btcData = data.snapshots.find((s) => s.symbol === "BTC_USD")
      if (btcData) {
        const firstMetric = btcData.metrics[0]
        if (firstMetric) {
          setSelectedMetric({
            name: firstMetric.name,
            formattedName: firstMetric.formattedName
          })
        } else {
          setSelectedMetric({
            name: "BTC_PRICE",
            formattedName: "Bitcoin Price"
          })
        }
      }
    }
  }, [data, selectedMetric])

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-primary" />
      </div>
    )
  }

  if (error || !data) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <p className="text-danger text-lg">Failed to load dashboard</p>
          <p className="text-muted-foreground text-sm mt-2">{error}</p>
        </div>
      </div>
    )
  }

  const btcData = data.snapshots.find((s) => s.symbol === "BTC_USD")
  const ethData = data.snapshots.find((s) => s.symbol === "ETH_USD")

  return (
    <div className="min-h-screen p-4 md:p-8 animate-in fade-in duration-700">

      <div className="max-w-7xl mx-auto space-y-8">
        {/* Header */}
        <header className="text-center space-y-2 animate-in slide-in-from-top duration-500">
          <h1 className="text-4xl md:text-5xl font-bold text-balance bg-linear-to-r from-foreground to-foreground/70 bg-clip-text text-transparent">
            Crypto Market Intelligence
          </h1>
          <p className="text-muted-foreground text-lg">Real-time market data and insights</p>
        </header>

        {/* Hero Section - Current Market Snapshot */}
        <section className="space-y-6">
          <h2 className="text-2xl font-semibold text-foreground">Market Snapshot</h2>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {/* BTC & ETH Cards */}
            {btcData && (
              <PriceCard
                data={{
                  ...btcData,
                  name: btcData.symbol
                }}                
                onMetricClick={(name, formattedName) => setSelectedMetric({ name, formattedName })}
              />
            )}
            {ethData && (
              <PriceCard
                data={{
                  ...ethData,
                  name: ethData.symbol
                }}
                onMetricClick={(name, formattedName) => setSelectedMetric({ name, formattedName })}
              />
            )}
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            {/* Fear & Greed Index */}
            <div className="lg:col-span-1">
              <FearGreedGauge
                data={data.fear_greed.data}
                onClick={() =>
                  setSelectedMetric({
                    name: "FEAR_GREED",
                    formattedName: "Fear & Greed Index",
                  })
                }
              />
            </div>

            {/* Macro Metrics Grid */}
            <div className="lg:col-span-2">
              <MacroMetricsGrid
                data={data.macro_metrics.data}
                onMetricClick={(name, formattedName) => setSelectedMetric({ name, formattedName })}
              />
            </div>
          </div>
        </section>

        {/* Interactive Charts Section */}
        <section className="space-y-6">
          <h2 className="text-2xl font-semibold text-foreground">Historical Analysis</h2>
          <DualAxisChart dashboardData={data} selectedMetric={selectedMetric} onMetricChange={setSelectedMetric} />
        </section>
      </div>
    </div>
  )
}