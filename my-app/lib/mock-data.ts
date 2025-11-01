
const BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080"

export async function fetchDashboardData() {
  const response = await fetch(`${BASE_URL}/api/dashboard`)
  if (!response.ok) throw new Error("Failed to fetch dashboard data")
  return response.json()
}

export async function fetchHistoricalData(symbol: string, days = 365) {
  const response = await fetch(`${BASE_URL}/api/historical?symbol=${symbol}&days=${days}`)
  if (!response.ok) throw new Error("Failed to fetch historical data")
  return response.json()
}