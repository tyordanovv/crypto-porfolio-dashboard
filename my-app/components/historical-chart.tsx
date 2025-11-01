"use client"

import type React from "react"

import { useEffect, useState } from "react"
import { Area, AreaChart, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts"
import { Loader2 } from "lucide-react"
import { formatNumber } from "@/lib/format"

const BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080"

interface HistoricalChartProps {
  selectedMetric: {
    name: string
    formattedName: string
  } | null
}

const timeRanges = [
  { label: "7D", days: 7 },
  { label: "30D", days: 30 },
  { label: "90D", days: 90 },
  { label: "1Y", days: 365 },
  { label: "ALL", days: 9999 },
]

export function HistoricalChart({ selectedMetric }: HistoricalChartProps) {
  const [data, setData] = useState<any[]>([])
  const [loading, setLoading] = useState(false)
  const [selectedRange, setSelectedRange] = useState(30)

  useEffect(() => {
    if (!selectedMetric) return

    async function fetchHistoricalData() {
      setLoading(true)
      try {
        const response = await fetch(`${BASE_URL}/api/historical?symbol=${selectedMetric.name}&days=${selectedRange}`)
        if (!response.ok) throw new Error("Failed to fetch historical data")
        const result = await response.json()

        if (result.data && result.data[0]) {
          const formatted = result.data[0].values.map((v: [string, number, string]) => ({
            date: v[0],
            value: v[1],
          }))
          setData(formatted)
        }
      } catch (err) {
        console.error("[v0] Error fetching historical data:", err)
      } finally {
        setLoading(false)
      }
    }

    fetchHistoricalData()
  }, [selectedMetric, selectedRange])

  if (!selectedMetric) {
    return (
      <div className="glass rounded-xl p-12 text-center animate-in fade-in duration-500">
        <BarChart3Icon className="h-16 w-16 text-muted-foreground mx-auto mb-4" />
        <p className="text-muted-foreground text-lg">Select a metric to view historical data</p>
        <p className="text-muted-foreground/70 text-sm mt-2">Click on any card above to explore trends</p>
      </div>
    )
  }

  return (
    <div className="glass rounded-xl p-6 space-y-6 animate-in fade-in duration-500">
      {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <h3 className="text-lg font-semibold text-foreground">{selectedMetric.formattedName}</h3>
          <p className="text-sm text-muted-foreground">Historical trends</p>
        </div>

        {/* Time Range Selector */}
        <div className="flex gap-2">
          {timeRanges.map((range) => (
            <button
              key={range.label}
              onClick={() => setSelectedRange(range.days)}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                selectedRange === range.days
                  ? "bg-primary text-primary-foreground"
                  : "bg-muted text-muted-foreground hover:bg-muted/80"
              }`}
            >
              {range.label}
            </button>
          ))}
        </div>
      </div>

      {/* Chart */}
      <div className="h-96">
        {loading ? (
          <div className="h-full flex items-center justify-center">
            <Loader2 className="h-8 w-8 animate-spin text-primary" />
          </div>
        ) : data.length > 0 ? (
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={data}>
              <defs>
                <linearGradient id="colorValue" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="hsl(var(--primary))" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="hsl(var(--primary))" stopOpacity={0} />
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" opacity={0.3} />
              <XAxis
                dataKey="date"
                stroke="hsl(var(--muted-foreground))"
                fontSize={12}
                tickLine={false}
                axisLine={false}
              />
              <YAxis
                stroke="hsl(var(--muted-foreground))"
                fontSize={12}
                tickLine={false}
                axisLine={false}
                tickFormatter={(value) => formatNumber(value)}
              />
              <Tooltip
                content={({ active, payload }) => {
                  if (active && payload && payload.length) {
                    return (
                      <div className="glass rounded-lg p-3 border border-border">
                        <p className="text-xs text-muted-foreground">{payload[0].payload.date}</p>
                        <p className="text-sm font-semibold text-foreground mt-1">
                          {formatNumber(payload[0].value as number)}
                        </p>
                      </div>
                    )
                  }
                  return null
                }}
              />
              <Area
                type="monotone"
                dataKey="value"
                stroke="hsl(var(--primary))"
                strokeWidth={2}
                fill="url(#colorValue)"
                animationDuration={1000}
              />
            </AreaChart>
          </ResponsiveContainer>
        ) : (
          <div className="h-full flex items-center justify-center">
            <p className="text-muted-foreground">No data available</p>
          </div>
        )}
      </div>
    </div>
  )
}

function BarChart3Icon(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M3 3v18h18" />
      <path d="M18 17V9" />
      <path d="M13 17V5" />
      <path d="M8 17v-3" />
    </svg>
  )
}
