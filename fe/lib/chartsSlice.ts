import { createSlice, type PayloadAction } from "@reduxjs/toolkit"
import type { CryptoMetric } from "@/lib/types"

export interface ChartMetric {
  id: string
  asset: "BTC_USD" | "ETH_USD"
  metric: CryptoMetric
  color: string
}

export interface ChartState {
  id: string
  metrics: ChartMetric[]
  macroMetrics: Array<{ id: string; type: string; color: string }>
  scaleType: "linear" | "log"
}

interface ChartsState {
  charts: ChartState[]
}

const COLORS = ["#a78bfa", "#22d3ee", "#fb923c", "#34d399", "#f472b6"]

const initialState: ChartsState = {
  charts: [
    {
      id: "chart-btc",
      metrics: [
        {
          id: "1",
          asset: "BTC_USD",
          metric: "price",
          color: COLORS[0],
        },
      ],
      macroMetrics: [],
      scaleType: "linear",
    },
    {
      id: "chart-eth",
      metrics: [
        {
          id: "2",
          asset: "ETH_USD",
          metric: "price",
          color: COLORS[1],
        },
      ],
      macroMetrics: [],
      scaleType: "linear",
    },
  ],
}

const chartsSlice = createSlice({
  name: "charts",
  initialState,
  reducers: {
    addChart: (state) => {
      const newChart: ChartState = {
        id: `chart-${Date.now()}`,
        metrics: [],
        macroMetrics: [],
        scaleType: "linear",
      }
      state.charts.push(newChart)
    },
    removeChart: (state, action: PayloadAction<string>) => {
      if (state.charts.length > 1) {
        state.charts = state.charts.filter((chart) => chart.id !== action.payload)
      }
    },
    addMetric: (state, action: PayloadAction<{ chartId: string; metric: ChartMetric }>) => {
      const chart = state.charts.find((c) => c.id === action.payload.chartId)
      if (chart && chart.metrics.length < 5) {
        chart.metrics.push(action.payload.metric)
      }
    },
    removeMetric: (state, action: PayloadAction<{ chartId: string; metricId: string }>) => {
      const chart = state.charts.find((c) => c.id === action.payload.chartId)
      if (chart) {
        chart.metrics = chart.metrics.filter((m) => m.id !== action.payload.metricId)
      }
    },
    addMacroMetric: (
      state,
      action: PayloadAction<{ chartId: string; macro: { id: string; type: string; color: string } }>,
    ) => {
      const chart = state.charts.find((c) => c.id === action.payload.chartId)
      if (chart && chart.macroMetrics.length < 2) {
        chart.macroMetrics.push(action.payload.macro)
      }
    },
    removeMacroMetric: (state, action: PayloadAction<{ chartId: string; macroId: string }>) => {
      const chart = state.charts.find((c) => c.id === action.payload.chartId)
      if (chart) {
        chart.macroMetrics = chart.macroMetrics.filter((m) => m.id !== action.payload.macroId)
      }
    },
    toggleScaleType: (state, action: PayloadAction<string>) => {
      const chart = state.charts.find((c) => c.id === action.payload)
      if (chart) {
        chart.scaleType = chart.scaleType === "linear" ? "log" : "linear"
      }
    },
  },
})

export const { addChart, removeChart, addMetric, removeMetric, addMacroMetric, removeMacroMetric, toggleScaleType } =
  chartsSlice.actions
export default chartsSlice.reducer
