export function formatNumber(num: number): string {
  if (num >= 1e9) {
    return (num / 1e9).toFixed(2) + "B"
  }
  if (num >= 1e6) {
    return (num / 1e6).toFixed(2) + "M"
  }
  if (num >= 1e3) {
    return (num / 1e3).toFixed(2) + "K"
  }
  return num.toFixed(2)
}

export function formatPercentage(num: number): string {
  const sign = num >= 0 ? "+" : ""
  return `${sign}${num.toFixed(2)}%`
}

export function calculateChange(data: Array<[number, number, number]>, hours: number): number {
  if (data.length < 2) return 0

  const latest = data[data.length - 1][1]
  const hoursAgo = Date.now() - hours * 60 * 60 * 1000

  // Find the closest data point to the target time
  let closest = data[0]
  for (const point of data) {
    if (Math.abs(point[0] - hoursAgo) < Math.abs(closest[0] - hoursAgo)) {
      closest = point
    }
  }

  const previous = closest[1]
  return ((latest - previous) / previous) * 100
}
