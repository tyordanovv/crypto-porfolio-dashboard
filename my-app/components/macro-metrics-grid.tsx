"use client"

import { formatNumber } from "@/lib/format"
import { TrendingUp, DollarSign, Percent, Users, BarChart3, Activity } from "lucide-react"

interface MacroMetricsGridProps {
  data: Array<{
    name: string
    formattedName: string
    values: Array<[string, number, string]>
  }>
  onMetricClick: (name: string, formattedName: string) => void
}

const iconMap: Record<string, any> = {
  DGS10: Percent,
  DGS2: Percent,
  FEDFUNDS: DollarSign,
  UNRATE: Users,
  CPIAUCSL: TrendingUp,
  default: BarChart3,
}

export function MacroMetricsGrid({ data, onMetricClick }: MacroMetricsGridProps) {
  return (
    <div className="glass rounded-xl p-6 space-y-4 animate-in fade-in slide-in-from-bottom duration-700 delay-100">
      <h3 className="text-sm text-muted-foreground">Economic Indicators</h3>

      <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
        {data.slice(0, 6).map((metric) => {
          const latest = metric.values[metric.values.length - 1]
          const value = latest[1]
          const date = latest[0]
          const Icon = iconMap[metric.name] || iconMap.default

          return (
            <button
              key={metric.name}
              onClick={() => onMetricClick(metric.name, metric.formattedName)}
              className="glass-hover rounded-lg p-4 text-left space-y-2 group"
            >
              <div className="flex items-center justify-between">
                <Icon className="h-4 w-4 text-primary" />
                <Activity className="h-3 w-3 text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity" />
              </div>
              <div>
                <p className="text-xs text-muted-foreground truncate">{metric.formattedName}</p>
                <p className="text-lg font-bold text-foreground mt-1">
                  {formatNumber(value)}
                  {metric.name.includes("RATE") || metric.name.includes("DGS") ? "%" : ""}
                </p>
                <p className="text-xs text-muted-foreground/70 mt-1">{date}</p>
              </div>
            </button>
          )
        })}
      </div>
    </div>
  )
}
