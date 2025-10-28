"use client"

import React from "react"

import { useEffect } from "react"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import CryptoChart from "@/components/crypto-chart"
import MetricsDisplay from "@/components/metrics-display"
import { Moon, Sun, TrendingUp, Plus, Loader2 } from "lucide-react"
import { useAppDispatch, useAppSelector } from "@/lib/hooks"
import { loadDashboardData } from "@/lib/dashboardSlice"
import { addChart, removeChart } from "@/lib/chartsSlice"

export default function CryptoDashboard() {
  const dispatch = useAppDispatch()
  const { data: dashboardData, loading } = useAppSelector((state) => state.dashboard)
  const { charts } = useAppSelector((state) => state.charts)
  const [darkMode, setDarkMode] = React.useState(true)

  useEffect(() => {
    if (darkMode) {
      document.documentElement.classList.add("dark")
    } else {
      document.documentElement.classList.remove("dark")
    }
  }, [darkMode])

  useEffect(() => {
    dispatch(loadDashboardData())
  }, [dispatch])

  const handleAddChart = () => {
    dispatch(addChart())
  }

  const handleRemoveChart = (chartId: string) => {
    dispatch(removeChart(chartId))
  }

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b border-border bg-card/50 backdrop-blur-sm">
        <div className="mx-auto max-w-[1800px] px-4 py-4 md:px-6 lg:px-8">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-linear-to-br from-primary to-primary/70 shadow-lg">
                <TrendingUp className="h-7 w-7 text-primary-foreground" />
              </div>
              <div>
                <h1 className="text-2xl font-bold text-foreground">Crypto Analytics</h1>
                <p className="text-sm text-muted-foreground">Real-time market intelligence</p>
              </div>
            </div>
            <div className="flex items-center gap-2">
              <Button onClick={handleAddChart} className="gap-2 shadow-sm">
                <Plus className="h-4 w-4" />
                Add Chart
              </Button>
              <Button variant="outline" size="icon" onClick={() => setDarkMode(!darkMode)} className="h-10 w-10">
                {darkMode ? <Sun className="h-5 w-5" /> : <Moon className="h-5 w-5" />}
              </Button>
            </div>
          </div>
        </div>
      </header>

      <main className="mx-auto max-w-[1800px] px-4 py-6 md:px-6 lg:px-8">
        {/* Metrics Display */}
        {loading ? (
          <div className="mb-8 flex items-center justify-center py-16">
            <div className="text-center">
              <Loader2 className="mx-auto h-10 w-10 animate-spin text-primary" />
              <p className="mt-4 text-sm text-muted-foreground">Loading market data...</p>
            </div>
          </div>
        ) : dashboardData ? (
          <div className="mb-8">
            <MetricsDisplay snapshots={dashboardData.snapshots} macroMetrics={dashboardData.macro_metrics} />
          </div>
        ) : null}

        {/* Charts */}
        <div className="space-y-6">
          <div className="flex items-center gap-2">
            <div className="h-1 w-12 rounded-full bg-primary" />
            <h2 className="text-xl font-bold text-foreground">Charts</h2>
          </div>
          <div className="grid gap-6 lg:grid-cols-2">
            {charts.map((chart) => (
              <Card key={chart.id} className="p-6 shadow-sm">
                <CryptoChart chartId={chart.id} onRemove={handleRemoveChart} />
              </Card>
            ))}
          </div>
        </div>
      </main>
    </div>
  )
}
