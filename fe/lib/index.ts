import { configureStore } from "@reduxjs/toolkit"
import dashboardReducer from "./dashboardSlice"
import chartsReducer from "./chartsSlice"

export const store = configureStore({
  reducer: {
    dashboard: dashboardReducer,
    charts: chartsReducer,
  },
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
