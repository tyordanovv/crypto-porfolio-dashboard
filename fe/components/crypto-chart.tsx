"use client"

import { useEffect, useState } from "react"
import {
  LineChart,
  Line,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Legend,
  ReferenceLine,
  ResponsiveContainer,
} from "recharts"
import { ChartContainer, ChartTooltip } from "@/components/ui/chart"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Loader2, X, Plus } from "lucide-react"
import { useAppDispatch, useAppSelector } from "@/lib/hooks"
import { addMetric, removeMetric, addMacroMetric, removeMacroMetric, toggleScaleType } from "@/lib/chartsSlice"
import { fetchHistoricalMacroData, parseAssetData } from "@/lib/crypto-api"
import type { CryptoMetric, MacroMetricType, ChartDataPoint } from "@/lib/types"

const COLORS = ["#a78bfa", "#22d3ee", "#fb923c", "#34d399", "#f472b6"]

const METRIC_LABELS: Record<CryptoMetric, string> = {
  price: "Price (USD)",
  volume: "Volume (USD)",
  market_cap: "Market Cap (USD)",
}

const MACRO_LABELS: Record<MacroMetricType, string> = {
  fear_and_greed: "Fear & Greed Index",
  CPIAUCSL: "CPI (Inflation)",
  DEXUSEU: "USD/EUR Exchange Rate",
  DFF: "Federal Funds Rate",
  DGS10: "10-Year Treasury",
  DGS2: "2-Year Treasury",
  FEDFUNDS: "Fed Funds Rate",
  M2SL: "M2 Money Supply",
  T10Y2Y: "10Y-2Y Spread",
  UNRATE: "Unemployment Rate",
}

interface CryptoChartProps {
  chartId: string
  onRemove: (chartId: string) => void
}

const CustomTooltip = ({ active, payload, label, chartConfig }: any) => {
  if (!active || !payload || !payload.length) return null

  return (
    <div className="rounded-lg border border-border bg-background p-3 shadow-lg">
      <p className="mb-2 text-sm font-semibold text-foreground">
        {new Date(label).toLocaleDateString("en-US", {
          month: "short",
          day: "numeric",
          year: "numeric",
        })}
      </p>
      <div className="space-y-1">
        {payload.map((entry: any, index: number) => {
          const config = chartConfig[entry.dataKey]
          if (!config) return null

          const formatValue = (value: number, dataKey: string) => {
            if (dataKey.startsWith("macro_")) {
              return value.toFixed(2)
            }
            if (value >= 1e9) return `$${(value / 1e9).toFixed(2)}B`
            if (value >= 1e6) return `$${(value / 1e6).toFixed(2)}M`
            if (value >= 1e3) return `$${(value / 1e3).toFixed(2)}K`
            return `$${value.toFixed(2)}`
          }

          return (
            <div key={index} className="flex items-center gap-2 text-sm">
              <div className="h-2 w-2 rounded-full" style={{ backgroundColor: config.color }} />
              <span className="font-medium text-foreground">{config.label}:</span>
              <span className="text-muted-foreground">{formatValue(entry.value, entry.dataKey)}</span>
            </div>
          )
        })}
      </div>
    </div>
  )
}

export default function CryptoChart({ chartId, onRemove }: CryptoChartProps) {
  const dispatch = useAppDispatch()
  const dashboardData = useAppSelector((state) => state.dashboard.data)
  const chartState = useAppSelector((state) => state.charts.charts.find((c) => c.id === chartId))
  const allCharts = useAppSelector((state) => state.charts.charts)

  const [chartData, setChartData] = useState<ChartDataPoint[]>([])
  const [loading, setLoading] = useState(false)
  const [selectedAsset, setSelectedAsset] = useState<"BTC_USD" | "ETH_USD">("BTC_USD")
  const [selectedMetric, setSelectedMetric] = useState<CryptoMetric>("price")
  const [selectedMacroMetric, setSelectedMacroMetric] = useState<MacroMetricType>("fear_and_greed")

  useEffect(() => {
    if (dashboardData && chartState) {
      loadChartData()
    }
  }, [dashboardData])

  const loadChartData = async () => {
    if (!dashboardData || !chartState || (chartState.metrics.length === 0 && chartState.macroMetrics.length === 0)) {
      setChartData([])
      return
    }

    setLoading(true)
    try {
      const mergedData: Record<number, any> = {}

      chartState.metrics.forEach((metric) => {
        const snapshot = dashboardData.snapshots.find((s) => s.symbol === metric.asset)
        if (snapshot) {
          const data = parseAssetData(snapshot, metric.metric)
          const key = `${metric.asset}_${metric.metric}`

          data.forEach((point) => {
            if (!mergedData[point.timestamp]) {
              mergedData[point.timestamp] = { timestamp: point.timestamp }
            }
            mergedData[point.timestamp][key] = point.value
          })
        }
      })

      const macroPromises = chartState.macroMetrics.map((macro) =>
        fetchHistoricalMacroData(macro.type as MacroMetricType, 64),
      )
      const macroResults = await Promise.all(macroPromises)

      macroResults.forEach((data, index) => {
        const macro = chartState.macroMetrics[index]
        const key = `macro_${macro.type}`

        data.forEach((point) => {
          if (!mergedData[point.timestamp]) {
            mergedData[point.timestamp] = { timestamp: point.timestamp }
          }
          mergedData[point.timestamp][key] = point.value
        })
      })

      const sortedData = Object.values(mergedData).sort((a, b) => a.timestamp - b.timestamp)
      setChartData(sortedData)
    } catch (error) {
      console.error("Error loading chart data:", error)
    } finally {
      setLoading(false)
    }
  }

  const handleAddMetric = () => {
    if (!chartState || chartState.metrics.length >= 5) return

    const newMetric = {
      id: Date.now().toString(),
      asset: selectedAsset,
      metric: selectedMetric,
      color: COLORS[chartState.metrics.length % COLORS.length],
    }

    dispatch(addMetric({ chartId, metric: newMetric }))
  }

  const handleRemoveMetric = (metricId: string) => {
    dispatch(removeMetric({ chartId, metricId }))
  }

  const handleAddMacroMetric = () => {
    if (!chartState || chartState.macroMetrics.length >= 2) return

    const newMacro = {
      id: Date.now().toString(),
      type: selectedMacroMetric,
      color: chartState.macroMetrics.length === 0 ? "#ef4444" : "#f59e0b",
    }

    dispatch(addMacroMetric({ chartId, macro: newMacro }))
  }

  const handleRemoveMacroMetric = (macroId: string) => {
    dispatch(removeMacroMetric({ chartId, macroId }))
  }

  const handleToggleScale = () => {
    dispatch(toggleScaleType(chartId))
  }

  if (!chartState) return null

  const formatXAxis = (timestamp: number) => {
    const date = new Date(timestamp)
    return date.toLocaleDateString("en-US", { month: "short", day: "numeric" })
  }

  const formatYAxis = (value: number) => {
    if (value >= 1e9) return `$${(value / 1e9).toFixed(1)}B`
    if (value >= 1e6) return `$${(value / 1e6).toFixed(1)}M`
    if (value >= 1e3) return `$${(value / 1e3).toFixed(1)}K`
    return `$${value.toFixed(0)}`
  }

  const chartConfig = {
    ...chartState.metrics.reduce(
      (config, metric) => {
        const key = `${metric.asset}_${metric.metric}`
        config[key] = {
          label: `${metric.asset.split("_")[0]} ${METRIC_LABELS[metric.metric]}`,
          color: metric.color,
        }
        return config
      },
      {} as Record<string, { label: string; color: string }>,
    ),
    ...chartState.macroMetrics.reduce(
      (config, macro) => {
        const key = `macro_${macro.type}`
        config[key] = {
          label: MACRO_LABELS[macro.type as MacroMetricType],
          color: macro.color,
        }
        return config
      },
      {} as Record<string, { label: string; color: string }>,
    ),
  }

  const hasMacroMetrics = chartState.macroMetrics.length > 0

  return (
    <div className="space-y-4">
      {/* Chart Header */}
      <div className="flex flex-wrap items-center justify-between gap-4">
        <div className="flex flex-wrap items-center gap-2">
          {chartState.metrics.map((metric) => (
            <Badge key={metric.id} variant="secondary" className="flex items-center gap-2 px-3 py-1">
              <div className="h-2 w-2 rounded-full" style={{ backgroundColor: metric.color }} />
              <span className="text-xs font-medium">
                {metric.asset.split("_")[0]} {metric.metric.charAt(0).toUpperCase() + metric.metric.slice(1)}
              </span>
              <button onClick={() => handleRemoveMetric(metric.id)} className="ml-1 hover:text-destructive">
                <X className="h-3 w-3" />
              </button>
            </Badge>
          ))}
          {chartState.macroMetrics.map((macro) => (
            <Badge key={macro.id} variant="outline" className="flex items-center gap-2 border-red-500/50 px-3 py-1">
              <div className="h-2 w-2 rounded-full" style={{ backgroundColor: macro.color }} />
              <span className="text-xs font-medium">{MACRO_LABELS[macro.type as MacroMetricType]}</span>
              <button onClick={() => handleRemoveMacroMetric(macro.id)} className="ml-1 hover:text-destructive">
                <X className="h-3 w-3" />
              </button>
            </Badge>
          ))}
        </div>

        <div className="flex items-center gap-2">
          {allCharts.length > 1 && (
            <Button variant="ghost" size="icon" onClick={() => onRemove(chartId)} className="h-8 w-8">
              <X className="h-4 w-4" />
            </Button>
          )}
          <Button variant="outline" size="sm" onClick={handleToggleScale}>
            {chartState.scaleType === "linear" ? "Linear" : "Log"} Scale
          </Button>
        </div>
      </div>

      <div className="space-y-3">
        {/* Asset Metrics Block */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-3 text-sm font-semibold text-foreground">Asset Metrics</h3>
          <div className="flex flex-wrap items-end gap-3">
            <div className="min-w-[140px] flex-1 space-y-2">
              <label className="text-xs font-medium text-muted-foreground">Asset</label>
              <Select value={selectedAsset} onValueChange={(value: "BTC_USD" | "ETH_USD") => setSelectedAsset(value)}>
                <SelectTrigger className="w-full bg-background">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="BTC_USD">Bitcoin (BTC)</SelectItem>
                  <SelectItem value="ETH_USD">Ethereum (ETH)</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="min-w-[140px] flex-1 space-y-2">
              <label className="text-xs font-medium text-muted-foreground">Metric</label>
              <Select value={selectedMetric} onValueChange={(value: CryptoMetric) => setSelectedMetric(value)}>
                <SelectTrigger className="w-full bg-background">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="price">Price</SelectItem>
                  <SelectItem value="volume">Volume</SelectItem>
                  <SelectItem value="market_cap">Market Cap</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <Button onClick={handleAddMetric} disabled={chartState.metrics.length >= 5} className="gap-2">
              <Plus className="h-4 w-4" />
              Add Metric
            </Button>
          </div>
        </div>

        {/* Macro Metrics Block */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-3 text-sm font-semibold text-foreground">Macro Metrics</h3>
          <div className="flex flex-wrap items-end gap-3">
            <div className="min-w-[200px] flex-1 space-y-2">
              <label className="text-xs font-medium text-muted-foreground">Index</label>
              <Select
                value={selectedMacroMetric}
                onValueChange={(value: MacroMetricType) => setSelectedMacroMetric(value)}
              >
                <SelectTrigger className="w-full bg-background">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="fear_and_greed">Fear & Greed Index</SelectItem>
                  <SelectItem value="CPIAUCSL">CPI (Inflation)</SelectItem>
                  <SelectItem value="DEXUSEU">USD/EUR Exchange Rate</SelectItem>
                  <SelectItem value="DFF">Federal Funds Rate</SelectItem>
                  <SelectItem value="DGS10">10-Year Treasury</SelectItem>
                  <SelectItem value="DGS2">2-Year Treasury</SelectItem>
                  <SelectItem value="M2SL">M2 Money Supply</SelectItem>
                  <SelectItem value="T10Y2Y">10Y-2Y Spread</SelectItem>
                  <SelectItem value="UNRATE">Unemployment Rate</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <Button
              onClick={handleAddMacroMetric}
              disabled={chartState.macroMetrics.length >= 2}
              variant="secondary"
              className="gap-2"
            >
              <Plus className="h-4 w-4" />
              Add Macro Metric
            </Button>
          </div>
        </div>
      </div>

      {/* Chart */}
      <div className="relative h-[400px] w-full">
        {loading && (
          <div className="absolute inset-0 z-10 flex items-center justify-center bg-background/80">
            <Loader2 className="h-8 w-8 animate-spin text-primary" />
          </div>
        )}

        {chartState.metrics.length === 0 && chartState.macroMetrics.length === 0 ? (
          <div className="flex h-full items-center justify-center text-muted-foreground">
            <p>Add a metric to start visualizing data</p>
          </div>
        ) : (
          <ChartContainer config={chartConfig} className="h-full w-full">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={chartData}>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="timestamp"
                  tickFormatter={formatXAxis}
                  className="text-xs"
                  stroke="hsl(var(--muted-foreground))"
                />
                <YAxis
                  yAxisId="left"
                  scale={chartState.scaleType}
                  domain={chartState.scaleType === "log" ? ["auto", "auto"] : undefined}
                  tickFormatter={formatYAxis}
                  className="text-xs"
                  stroke="hsl(var(--muted-foreground))"
                />
                {hasMacroMetrics && (
                  <YAxis
                    yAxisId="right"
                    orientation="right"
                    domain={["auto", "auto"]}
                    className="text-xs"
                    stroke="#ef4444"
                  />
                )}
                <ChartTooltip content={<CustomTooltip chartConfig={chartConfig} />} />
                <Legend wrapperStyle={{ fontSize: "12px" }} />
                {hasMacroMetrics && chartState.macroMetrics.some((m) => m.type === "fear_and_greed") && (
                  <>
                    <ReferenceLine yAxisId="right" y={25} stroke="#ef4444" strokeDasharray="3 3" strokeOpacity={0.3} />
                    <ReferenceLine yAxisId="right" y={75} stroke="#22c55e" strokeDasharray="3 3" strokeOpacity={0.3} />
                  </>
                )}
                {chartState.metrics.map((metric) => (
                  <Line
                    key={metric.id}
                    yAxisId="left"
                    type="monotone"
                    dataKey={`${metric.asset}_${metric.metric}`}
                    stroke={metric.color}
                    strokeWidth={3}
                    dot={false}
                    activeDot={{ r: 4 }}
                  />
                ))}
                {chartState.macroMetrics.map((macro) => (
                  <Area
                    key={macro.id}
                    yAxisId="right"
                    type="monotone"
                    dataKey={`macro_${macro.type}`}
                    stroke={macro.color}
                    fill={macro.color}
                    fillOpacity={0.2}
                    strokeWidth={2}
                    dot={false}
                    activeDot={{ r: 4 }}
                  />
                ))}
              </LineChart>
            </ResponsiveContainer>
          </ChartContainer>
        )}
      </div>
    </div>
  )
}
