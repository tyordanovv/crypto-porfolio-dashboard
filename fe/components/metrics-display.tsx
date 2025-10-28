"use client"

import { Card } from "@/components/ui/card"
import { TrendingUp, TrendingDown, DollarSign, Activity, Percent } from "lucide-react"
import type { AssetSnapshot, MacroMetricData } from "@/lib/types"

interface MetricsDisplayProps {
  snapshots: AssetSnapshot[]
  macroMetrics: MacroMetricData[]
}

export default function MetricsDisplay({ snapshots, macroMetrics }: MetricsDisplayProps) {
  const formatMetricValue = (name: string, value: number) => {
    if (name.includes("RETURN") || name.includes("DOMINANCE")) {
      return `${(value * 100).toFixed(2)}%`
    }
    if (name.includes("RATIO")) {
      return value.toFixed(2)
    }
    if (value >= 1000) {
      return value.toLocaleString(undefined, { maximumFractionDigits: 2 })
    }
    return value.toFixed(2)
  }

  const getMetricIcon = (name: string, value: number) => {
    if (name.includes("RETURN")) {
      return value >= 0 ? <TrendingUp className="h-5 w-5" /> : <TrendingDown className="h-5 w-5" />
    }
    if (name.includes("DOMINANCE")) {
      return <Percent className="h-5 w-5" />
    }
    if (name.includes("RATIO")) {
      return <Activity className="h-5 w-5" />
    }
    return <DollarSign className="h-5 w-5" />
  }

  const getMetricColor = (name: string, value: number) => {
    if (name.includes("RETURN")) {
      return value >= 0 ? "text-green-500" : "text-red-500"
    }
    return "text-primary"
  }

  return (
    <div className="space-y-6">
      {/* Asset Metrics */}
      <div>
        <div className="mb-4 flex items-center gap-2">
          <div className="h-1 w-12 rounded-full bg-primary" />
          <h2 className="text-xl font-bold text-foreground">Asset Metrics</h2>
        </div>
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
          {snapshots.map((snapshot) =>
            snapshot.metrics.map((metric) => {
              const displayName = metric.formatedName
              const icon = getMetricIcon(metric.name, metric.value)
              const colorClass = getMetricColor(metric.name, metric.value)

              return (
                <Card key={`${snapshot.symbol}-${metric.name}`} className="group relative overflow-hidden p-6">
                  <div className="absolute right-0 top-0 h-24 w-24 translate-x-8 -translate-y-8 rounded-full bg-primary/5" />
                  <div className="relative space-y-3">
                    <div className="flex items-center justify-between">
                      <div className={`rounded-lg bg-primary/10 p-2 ${colorClass}`}>{icon}</div>
                      <span className="text-xs font-medium text-muted-foreground">
                        {snapshot.symbol.replace("_", "/")}
                      </span>
                    </div>
                    <div>
                      <p className="text-sm font-medium text-muted-foreground">{displayName}</p>
                      <p className="mt-1 text-2xl font-bold text-foreground">
                        {formatMetricValue(metric.name, metric.value)}
                      </p>
                    </div>
                  </div>
                </Card>
              )
            }),
          )}
        </div>
      </div>

      {/* Macro Metrics */}
      <div>
        <div className="mb-4 flex items-center gap-2">
          <div className="h-1 w-12 rounded-full bg-primary" />
          <h2 className="text-xl font-bold text-foreground">Macro Metrics</h2>
        </div>
        <div className="grid gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5">
          {macroMetrics.map((metric) => (
            <Card key={metric.name} className="group relative overflow-hidden p-5">
              <div className="absolute right-0 top-0 h-20 w-20 translate-x-6 -translate-y-6 rounded-full bg-primary/5" />
              <div className="relative space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-xs font-semibold uppercase tracking-wider text-primary">{metric.name}</span>
                </div>
                <p className="text-sm font-medium text-muted-foreground">{metric.formattedName}</p>
                <p className="text-2xl font-bold text-foreground">{metric.value.toLocaleString()}</p>
              </div>
            </Card>
          ))}
        </div>
      </div>
    </div>
  )
}
