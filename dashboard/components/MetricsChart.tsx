'use client'

import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts'

interface Opportunity {
  strategy: string
  symbol: string
  venue_a: string
  venue_b: string
  spread_bps: number
  estimated_profit: number
  timestamp: string
}

export default function MetricsChart({ opportunities }: { opportunities: Opportunity[] }) {
  // Group opportunities by hour for chart
  const hourlyData = opportunities.reduce((acc: Record<string, { time: string; count: number; profit: number }>, opp) => {
    const hour = new Date(opp.timestamp).toISOString().slice(0, 13) + ':00'
    if (!acc[hour]) {
      acc[hour] = { time: hour, count: 0, profit: 0 }
    }
    acc[hour].count += 1
    acc[hour].profit += opp.estimated_profit
    return acc
  }, {})

  const chartData = Object.values(hourlyData).slice(-24) // Last 24 hours

  if (chartData.length === 0) {
    return <p className="text-gray-500">No data to display yet.</p>
  }

  return (
    <ResponsiveContainer width="100%" height={300}>
      <LineChart data={chartData}>
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis dataKey="time" />
        <YAxis yAxisId="left" />
        <YAxis yAxisId="right" orientation="right" />
        <Tooltip />
        <Legend />
        <Line yAxisId="left" type="monotone" dataKey="count" stroke="#8884d8" name="Opportunities" />
        <Line yAxisId="right" type="monotone" dataKey="profit" stroke="#82ca9d" name="Estimated Profit ($)" />
      </LineChart>
    </ResponsiveContainer>
  )
}
