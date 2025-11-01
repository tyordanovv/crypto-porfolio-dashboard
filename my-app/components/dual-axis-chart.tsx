"use client"

import { useEffect, useState, useMemo } from "react"
import { ComposedChart, Area, Line, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis, Legend } from "recharts"
import { Loader2, TrendingUp, Search, LogOut } from "lucide-react"
import { formatNumber } from "@/lib/format"

interface DualAxisChartProps {
  dashboardData: {
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
  selectedMetric: {
    name: string
    formattedName: string
  } | null
  onMetricChange: (metric: { name: string; formattedName: string } | null) => void
}

const timeRanges = [
  { label: "7D", days: 7 },
  { label: "30D", days: 30 },
  { label: "90D", days: 90 },
  { label: "180D", days: 180 },
  { label: "1Y", days: 365 },
  { label: "ALL", days: 9999 },
]

export function DualAxisChart({ dashboardData, selectedMetric, onMetricChange }: DualAxisChartProps) {
  const [selectedAsset, setSelectedAsset] = useState<"BTC_USD" | "ETH_USD">("BTC_USD")
  const [selectedRange, setSelectedRange] = useState(30)
  const [searchQuery, setSearchQuery] = useState("")
  const [showMetricDropdown, setShowMetricDropdown] = useState(false)
  const [logScale, setLogScale] = useState(false)

  // Get all available metrics for the dropdown
  const allMetrics = useMemo(() => {
    const metrics: Array<{ name: string; formattedName: string; category: string }> = []

    // Add BTC metrics
    const btcSnapshot = dashboardData.snapshots.find((s) => s.symbol === "BTC_USD")
    btcSnapshot?.metrics.forEach((m) => {
      metrics.push({ name: m.name, formattedName: m.formattedName, category: "Bitcoin" })
    })

    // Add ETH metrics
    const ethSnapshot = dashboardData.snapshots.find((s) => s.symbol === "ETH_USD")
    ethSnapshot?.metrics.forEach((m) => {
      metrics.push({ name: m.name, formattedName: m.formattedName, category: "Ethereum" })
    })

    // Add macro metrics
    dashboardData.macro_metrics.data.forEach((m) => {
      metrics.push({ name: m.name, formattedName: m.formattedName, category: "Macro" })
    })

    // Add fear & greed and its moving averages (using pre-computed values from backend)
    metrics.push({ name: "FEAR_GREED", formattedName: "Fear & Greed Index", category: "Sentiment" })
    metrics.push({ name: "FEAR_GREED_7D_MA", formattedName: "Fear & Greed Index - 7D MA", category: "Sentiment" })
    metrics.push({ name: "FEAR_GREED_14D_MA", formattedName: "Fear & Greed Index - 14D MA", category: "Sentiment" })
    metrics.push({ name: "FEAR_GREED_21D_MA", formattedName: "Fear & Greed Index - 21D MA", category: "Sentiment" })

    return metrics
  }, [dashboardData])

  // Filter metrics based on search
  const filteredMetrics = useMemo(() => {
    if (!searchQuery) return allMetrics
    const query = searchQuery.toLowerCase()
    return allMetrics.filter(
      (m) => m.formattedName.toLowerCase().includes(query) || m.name.toLowerCase().includes(query),
    )
  }, [allMetrics, searchQuery])

  // Get asset price data from cached dashboard data
  const assetPriceData = useMemo(() => {
    const snapshot = dashboardData.snapshots.find((s) => s.symbol === selectedAsset)
    if (!snapshot) return []

    return snapshot.prices.data.map(([timestamp, price]) => ({
      timestamp,
      date: new Date(timestamp).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
      price: logScale && price > 0 ? Math.log(price) : price,
      originalPrice: price,
    }))
  }, [dashboardData, selectedAsset, logScale])

  // Get metric data from cached dashboard data (using pre-computed moving averages)
  const getMetricData = useMemo(() => {
    if (!selectedMetric) return []

    // Handle Fear & Greed moving averages (using pre-computed values from backend)
    if (selectedMetric.name === "FEAR_GREED_7D_MA") {
      return dashboardData.fear_greed.data.map(([timestamp, , avg7d]) => ({
        timestamp,
        value: avg7d,
        date: new Date(timestamp).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
      }))
    }
    if (selectedMetric.name === "FEAR_GREED_14D_MA") {
      return dashboardData.fear_greed.data.map(([timestamp, , , avg14d]) => ({
        timestamp,
        value: avg14d,
        date: new Date(timestamp).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
      }))
    }
    if (selectedMetric.name === "FEAR_GREED_21D_MA") {
      return dashboardData.fear_greed.data.map(([timestamp, , , , avg21d]) => ({
        timestamp,
        value: avg21d,
        date: new Date(timestamp).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
      }))
    }

    // Handle original Fear & Greed
    if (selectedMetric.name === "FEAR_GREED") {
      return dashboardData.fear_greed.data.map(([timestamp, value]) => ({
        timestamp,
        value,
        date: new Date(timestamp).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
      }))
    }

    // For BTC metrics
    if (selectedMetric.name.startsWith("BTC_")) {
      const btcSnapshot = dashboardData.snapshots.find((s) => s.symbol === "BTC_USD")
      const metric = btcSnapshot?.metrics.find((m) => m.name === selectedMetric.name)
      if (metric) {
        return metric.data.map(([timestamp, value]) => ({
          timestamp,
          value,
          date: new Date(timestamp).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
        }))
      }
    }

    // For ETH metrics
    if (selectedMetric.name.startsWith("ETH_")) {
      const ethSnapshot = dashboardData.snapshots.find((s) => s.symbol === "ETH_USD")
      const metric = ethSnapshot?.metrics.find((m) => m.name === selectedMetric.name)
      if (metric) {
        return metric.data.map(([timestamp, value]) => ({
          timestamp,
          value,
          date: new Date(timestamp).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
        }))
      }
    }

    // For macro metrics
    const macroMetric = dashboardData.macro_metrics.data.find((m) => m.name === selectedMetric.name)
    if (macroMetric) {
      return macroMetric.values.map(([date, value]) => ({
        timestamp: new Date(date).getTime(),
        value,
        date: new Date(date).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
      }))
    }

    return []
  }, [selectedMetric, dashboardData])

  // Merge asset and metric data by timestamp
  const chartData = useMemo(() => {
    if (!selectedMetric || getMetricData.length === 0) return assetPriceData

    const metricMap = new Map(getMetricData.map((point) => [point.timestamp, point.value]))

    return assetPriceData.map((point) => ({
      ...point,
      metricValue: metricMap.get(point.timestamp) || null,
    }))
  }, [assetPriceData, getMetricData, selectedMetric])

  // Filter data by time range
  const filteredData = useMemo(() => {
    const cutoffTime = Date.now() - selectedRange * 24 * 60 * 60 * 1000
    return chartData.filter((d) => d.timestamp >= cutoffTime)
  }, [chartData, selectedRange])

  // Calculate dynamic Y-axis domain for price data
  const priceDomain = useMemo(() => {
    if (filteredData.length === 0) return [0, 100]
    
    const prices = filteredData.map(d => d.price).filter(price => price != null)
    if (prices.length === 0) return [0, 100]
    
    const minPrice = Math.min(...prices)
    const maxPrice = Math.max(...prices)
    
    // Add 5% padding to top and bottom for better visualization
    const padding = (maxPrice - minPrice) * 0.05
    const domainMin = Math.max(0, minPrice - padding) // Don't go below 0
    const domainMax = maxPrice + padding
    
    return [domainMin, domainMax]
  }, [filteredData])

  // Calculate dynamic Y-axis domain for metric data
  const metricDomain = useMemo(() => {
    if (!selectedMetric || filteredData.length === 0) return [0, 100]
    
    const metricValues = filteredData.map(d => d.metricValue).filter(value => value != null)
    if (metricValues.length === 0) return [0, 100]
    
    const minValue = Math.min(...metricValues)
    const maxValue = Math.max(...metricValues)
    
    // Add 10% padding to top and bottom for metrics
    const padding = (maxValue - minValue) * 0.1
    const domainMin = minValue - padding
    const domainMax = maxValue + padding
    
    return [domainMin, domainMax]
  }, [filteredData, selectedMetric])

  // Format numbers for log scale display
  const formatLogValue = (value: number) => {
    if (!logScale) return `$${formatNumber(value)}`
    return `$${formatNumber(Math.exp(value))}`
  }

  return (
    <div className="glass rounded-xl p-6 space-y-6 animate-in fade-in duration-500">
      {/* Controls */}
      <div className="flex flex-col lg:flex-row gap-4">
        {/* Asset Selector */}
        <div className="flex-1">
          <label className="text-sm text-gray-300 mb-2 block">Primary Asset</label>
          <div className="flex gap-2">
            <button
              onClick={() => setSelectedAsset("BTC_USD")}
              className={`flex-1 px-4 py-2.5 rounded-lg text-sm font-medium transition-all ${
                selectedAsset === "BTC_USD"
                  ? "bg-blue-600 text-white"
                  : "bg-gray-700 text-gray-300 hover:bg-gray-600"
              }`}
            >
              Bitcoin (BTC)
            </button>
            <button
              onClick={() => setSelectedAsset("ETH_USD")}
              className={`flex-1 px-4 py-2.5 rounded-lg text-sm font-medium transition-all ${
                selectedAsset === "ETH_USD"
                  ? "bg-blue-600 text-white"
                  : "bg-gray-700 text-gray-300 hover:bg-gray-600"
              }`}
            >
              Ethereum (ETH)
            </button>
          </div>
        </div>

        {/* Metric Selector */}
        <div className="flex-1 relative">
          <label className="text-sm text-gray-300 mb-2 block">Secondary Metric (Optional)</label>
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="text"
              placeholder={selectedMetric?.formattedName || "Search metrics..."}
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onFocus={() => setShowMetricDropdown(true)}
              className="w-full pl-10 pr-4 py-2.5 rounded-lg bg-gray-700 text-white placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 border border-gray-600"
            />
            {selectedMetric && (
              <button
                onClick={() => {
                  onMetricChange(null)
                  setSearchQuery("")
                }}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-white"
              >
                ✕
              </button>
            )}
          </div>

          {/* Dropdown */}
          {showMetricDropdown && (
            <>
              <div className="fixed inset-0 z-10" onClick={() => setShowMetricDropdown(false)} />
              <div className="absolute top-full left-0 right-0 mt-2 max-h-64 overflow-y-auto bg-gray-800 rounded-lg border border-gray-600 z-20 shadow-lg">
                {filteredMetrics.length > 0 ? (
                  filteredMetrics.map((metric) => (
                    <button
                      key={metric.name}
                      onClick={() => {
                        onMetricChange(metric)
                        setSearchQuery("")
                        setShowMetricDropdown(false)
                      }}
                      className="w-full px-4 py-2.5 text-left hover:bg-gray-700 transition-colors border-b border-gray-600 last:border-b-0"
                    >
                      <div className="text-sm font-medium text-white">{metric.formattedName}</div>
                      <div className="text-xs text-gray-400">{metric.category}</div>
                    </button>
                  ))
                ) : (
                  <div className="px-4 py-3 text-sm text-gray-400">No metrics found</div>
                )}
              </div>
            </>
          )}
        </div>

        {/* Time Range Selector */}
        <div className="flex-1">
          <label className="text-sm text-gray-300 mb-2 block">Time Range & Scale</label>
          <div className="flex gap-2 flex-wrap">
            {timeRanges.map((range) => (
              <button
                key={range.label}
                onClick={() => setSelectedRange(range.days)}
                className={`px-3 py-2 rounded-lg text-sm font-medium transition-all ${
                  selectedRange === range.days
                    ? "bg-blue-600 text-white"
                    : "bg-gray-700 text-gray-300 hover:bg-gray-600"
                }`}
              >
                {range.label}
              </button>
            ))}
            <button
              onClick={() => setLogScale(!logScale)}
              className={`px-3 py-2 rounded-lg text-sm font-medium transition-all flex items-center gap-2 ${
                logScale
                  ? "bg-purple-600 text-white"
                  : "bg-gray-700 text-gray-300 hover:bg-gray-600"
              }`}
              title="Logarithmic scale"
            >
              <LogOut className="h-4 w-4" />
              Log
            </button>
          </div>
        </div>
      </div>

      {/* Chart */}
      <div className="h-96">
        {filteredData.length > 0 ? (
          <ResponsiveContainer width="100%" height="100%">
            <ComposedChart data={filteredData}>
              <defs>
                <linearGradient id="colorPrice" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.4} />
                  <stop offset="95%" stopColor="#3b82f6" stopOpacity={0.1} />
                </linearGradient>
                <linearGradient id="colorMetric" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#10b981" stopOpacity={0.8} />
                  <stop offset="95%" stopColor="#10b981" stopOpacity={0.3} />
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#4b5563" opacity={0.3} />
              <XAxis
                dataKey="date"
                stroke="#9ca3af"
                fontSize={12}
                tickLine={false}
                axisLine={false}
                tick={{ fill: '#9ca3af' }}
              />
              <YAxis
                key="left-axis"
                yAxisId="left"
                stroke="#3b82f6"
                fontSize={12}
                tickLine={false}
                axisLine={false}
                tick={{ fill: '#3b82f6' }}
                tickFormatter={formatLogValue}
                domain={priceDomain}
                allowDataOverflow={false}
              />
              {selectedMetric && (
                <YAxis
                  key="right-axis"
                  yAxisId="right"
                  orientation="right"
                  stroke="#10b981"
                  fontSize={12}
                  tickLine={false}
                  axisLine={false}
                  tick={{ fill: '#10b981' }}
                  tickFormatter={(value) => formatNumber(value)}
                  domain={metricDomain}
                  allowDataOverflow={false}
                />
              )}
              <Tooltip
                contentStyle={{
                  backgroundColor: '#1f2937',
                  border: '1px solid #374151',
                  borderRadius: '8px',
                  color: 'white'
                }}
                labelStyle={{ color: '#d1d5db' }}
                content={({ active, payload }) => {
                  if (active && payload && payload.length) {
                    const priceValue = logScale 
                      ? Math.exp(payload[0].value as number)
                      : (payload[0].value as number)
                    
                    return (
                      <div className="bg-gray-800 rounded-lg p-3 border border-gray-600 shadow-lg">
                        <p className="text-xs text-gray-300 mb-2">{payload[0].payload.date}</p>
                        <div className="space-y-1">
                          <p className="text-sm font-semibold text-blue-400">
                            {selectedAsset === "BTC_USD" ? "BTC" : "ETH"}: ${formatNumber(priceValue)}
                            {logScale && <span className="text-xs text-gray-400 ml-1">(log scale)</span>}
                          </p>
                          {selectedMetric && payload[1]?.value && (
                            <p className="text-sm font-semibold text-green-400">
                              {selectedMetric.formattedName}: {formatNumber(payload[1].value as number)}
                            </p>
                          )}
                        </div>
                      </div>
                    )
                  }
                  return null
                }}
              />
              <Legend 
                wrapperStyle={{ 
                  paddingTop: "20px", 
                  color: '#d1d5db',
                  fontSize: '12px'
                }} 
                iconType="line" 
              />
              <Area
                key="price-area"
                yAxisId="left"
                type="linear"
                dataKey="price"
                name={selectedAsset === "BTC_USD" ? "Bitcoin Price" : "Ethereum Price"}
                stroke="#3b82f6"
                strokeWidth={2}
                fill="url(#colorPrice)"
                animationDuration={500}
                dot={false}
                connectNulls={true}
              />
              {selectedMetric && (
                <Line
                  key="metric-line"
                  yAxisId="right"
                  type="linear"
                  dataKey="metricValue"
                  name={selectedMetric.formattedName}
                  stroke="#10b981"
                  strokeWidth={2}
                  dot={false}
                  animationDuration={500}
                  connectNulls={true}
                />
              )}
            </ComposedChart>
          </ResponsiveContainer>
        ) : (
          <div className="h-full flex flex-col items-center justify-center gap-4">
            <TrendingUp className="h-16 w-16 text-gray-500" />
            <div className="text-center">
              <p className="text-gray-400 text-lg">No data available for selected range</p>
              <p className="text-gray-500 text-sm mt-2">Try selecting a different time range</p>
            </div>
          </div>
        )}
      </div>

      {/* Scale Info */}
      {logScale && (
        <div className="text-center">
          <p className="text-sm text-gray-400">
            Logarithmic scale enabled - better for visualizing percentage changes and long-term trends
          </p>
        </div>
      )}
    </div>
  )
}