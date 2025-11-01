"use client"

import { Area, AreaChart, ResponsiveContainer } from "recharts"

interface MiniSparklineProps {
  data: Array<{ timestamp: number; value: number }>
  color?: "success" | "danger" | "primary"
}

export function MiniSparkline({ data, color = "primary" }: MiniSparklineProps) {
  const colorMap = {
    success: "var(--success)",
    danger: "var(--danger)",
    primary: "var(--primary)",
  }

  const strokeColor = colorMap[color]

  return (
    <ResponsiveContainer width="100%" height="100%">
      <AreaChart data={data}>
        <defs>
          <linearGradient id={`gradient-${color}`} x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor={strokeColor} stopOpacity={0.3} />
            <stop offset="95%" stopColor={strokeColor} stopOpacity={0} />
          </linearGradient>
        </defs>
        <Area
          type="monotone"
          dataKey="value"
          stroke={strokeColor}
          strokeWidth={2}
          fill={`url(#gradient-${color})`}
          isAnimationActive={true}
          animationDuration={1000}
        />
      </AreaChart>
    </ResponsiveContainer>
  )
}
