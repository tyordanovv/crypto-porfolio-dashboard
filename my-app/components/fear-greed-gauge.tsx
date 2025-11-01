"use client"

interface FearGreedGaugeProps {
  data: Array<[number, number, number, number, number, string]>
  onClick: () => void
}

export function FearGreedGauge({ data, onClick }: FearGreedGaugeProps) {
  const latest = data[data.length - 1]
  const value = latest[1]
  const avg7d = latest[2]
  const avg14d = latest[3]
  const avg21d = latest[4]
  const classification = latest[5]

  const getColor = (val: number) => {
    if (val < 25) return "text-danger"
    if (val < 50) return "text-warning"
    if (val < 75) return "text-chart-2"
    return "text-success"
  }

  const getGradient = (val: number) => {
    if (val < 25) return "from-danger to-danger/50"
    if (val < 50) return "from-warning to-warning/50"
    if (val < 75) return "from-chart-2 to-chart-2/50"
    return "from-success to-success/50"
  }

  const rotation = (value / 100) * 180 - 90

  return (
    <button
      onClick={onClick}
      className="glass glass-hover rounded-xl p-6 space-y-6 w-full text-left animate-in fade-in slide-in-from-bottom duration-700"
    >
      <div>
        <h3 className="text-sm text-muted-foreground">Fear & Greed Index</h3>
        <p className={`text-5xl font-bold mt-2 ${getColor(value)}`}>{value}</p>
        <p className="text-sm text-muted-foreground mt-1 capitalize">{classification}</p>
      </div>

      {/* Gauge Visualization */}
      <div className="relative h-32 flex items-end justify-center">
        <div className="relative w-48 h-24 overflow-hidden">
          {/* Background Arc */}
          <div className="absolute inset-0 rounded-t-full border-8 border-muted" />

          {/* Colored Arc */}
          <div
            className={`absolute inset-0 rounded-t-full border-8 bg-gradient-to-r ${getGradient(value)} opacity-30`}
            style={{
              clipPath: `polygon(0 100%, 0 0, 100% 0, 100% 100%, ${50 + (value / 100) * 50}% 100%, 50% 50%)`,
            }}
          />

          {/* Needle */}
          <div
            className="absolute bottom-0 left-1/2 w-1 h-20 bg-foreground origin-bottom transition-transform duration-1000 ease-out"
            style={{ transform: `translateX(-50%) rotate(${rotation}deg)` }}
          >
            <div className="absolute -top-2 left-1/2 -translate-x-1/2 w-3 h-3 rounded-full bg-foreground" />
          </div>
        </div>
      </div>

      {/* Averages */}
      <div className="grid grid-cols-3 gap-2 pt-4 border-t border-border/50">
        <div>
          <p className="text-xs text-muted-foreground">7d Avg</p>
          <p className="text-sm font-semibold text-foreground">{avg7d}</p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground">14d Avg</p>
          <p className="text-sm font-semibold text-foreground">{avg14d}</p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground">21d Avg</p>
          <p className="text-sm font-semibold text-foreground">{avg21d}</p>
        </div>
      </div>

      <p className="text-xs text-muted-foreground/70 text-center">Alternative.me</p>
    </button>
  )
}
