import { createSlice, createAsyncThunk, type PayloadAction } from "@reduxjs/toolkit"
import { fetchDashboardData } from "@/lib/crypto-api"
import type { DashboardResponse } from "@/lib/types"

interface DashboardState {
  data: DashboardResponse | null
  loading: boolean
  error: string | null
}

const initialState: DashboardState = {
  data: null,
  loading: false,
  error: null,
}

export const loadDashboardData = createAsyncThunk("dashboard/loadData", async () => {
  const data = await fetchDashboardData()
  return data
})

const dashboardSlice = createSlice({
  name: "dashboard",
  initialState,
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(loadDashboardData.pending, (state) => {
        state.loading = true
        state.error = null
      })
      .addCase(loadDashboardData.fulfilled, (state, action: PayloadAction<DashboardResponse>) => {
        state.loading = false
        state.data = action.payload
      })
      .addCase(loadDashboardData.rejected, (state, action) => {
        state.loading = false
        state.error = action.error.message || "Failed to load dashboard data"
      })
  },
})

export default dashboardSlice.reducer
