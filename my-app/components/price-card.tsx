"use client"

import { TrendingUp, TrendingDown } from "lucide-react"
import { formatNumber, formatPercentage, calculateChange } from "@/lib/format"
import { MiniSparkline } from "@/components/mini-sparkline"

interface PriceCardProps {
  data: {
    symbol: string
    name: string
    prices: {
      data: Array<[number, number, number]>
    }
    metrics: Array<{
      name: string
      formattedName: string
      data: Array<[number, number]>
    }>
  }
  onMetricClick: (name: string, formattedName: string) => void
}

export function PriceCard({ data, onMetricClick }: PriceCardProps) {
  const priceData = data.prices.data
  const latestPrice = priceData[priceData.length - 1]
  const price = latestPrice[1]
  const volume = latestPrice[2]
  const change24h = calculateChange(priceData, 24)

  const isPositive = change24h >= 0

  return (
    <div className="glass glass-hover rounded-xl p-6 space-y-6 cursor-pointer animate-in fade-in slide-in-from-bottom duration-500">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <h3 className="text-sm text-muted-foreground">{data.name}</h3>
          <p className="text-4xl font-bold mt-1 text-foreground">${formatNumber(price)}</p>
        </div>
        <div
          className={`flex items-center gap-1 px-3 py-1 rounded-full ${
            isPositive ? "bg-success/20 text-success" : "bg-danger/20 text-danger"
          }`}
        >
          {isPositive ? <TrendingUp className="h-4 w-4" /> : <TrendingDown className="h-4 w-4" />}
          <span className="text-sm font-semibold">{formatPercentage(change24h)}</span>
        </div>
      </div>

      {/* Sparkline */}
      <div className="h-24">
        <MiniSparkline
          data={priceData.map((d) => ({ timestamp: d[0], value: d[1] }))}
          color={isPositive ? "success" : "danger"}
        />
      </div>

      {/* Volume */}
      <div className="pt-4 border-t border-border/50">
        <p className="text-xs text-muted-foreground">24h Volume</p>
        <p className="text-lg font-semibold text-foreground">${formatNumber(volume)}</p>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-3 gap-4 pt-4 border-t border-border/50">
        {data.metrics.slice(0, 3).map((metric) => {
          const latestValue = metric.data[metric.data.length - 1][1]
          return (
            <button
              key={metric.name}
              onClick={(e) => {
                e.stopPropagation()
                onMetricClick(metric.name, metric.formattedName)
              }}
              className="text-left hover:bg-primary/10 rounded-lg p-2 transition-colors"
            >
              <p className="text-xs text-muted-foreground truncate">{metric.formattedName}</p>
              <p className="text-sm font-semibold text-foreground mt-1">
                {formatNumber(latestValue)}
                {metric.name.includes("DOMINANCE") && "%"}
              </p>
            </button>
          )
        })}
      </div>
    </div>
  )
}
