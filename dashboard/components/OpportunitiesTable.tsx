'use client'

interface Opportunity {
  strategy: string
  symbol: string
  venue_a: string
  venue_b: string
  spread_bps: number
  estimated_profit: number
  timestamp: string
}

export default function OpportunitiesTable({ opportunities }: { opportunities: Opportunity[] }) {
  if (opportunities.length === 0) {
    return <p className="text-gray-500">No opportunities detected yet.</p>
  }

  return (
    <div className="overflow-x-auto">
      <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
        <thead className="bg-gray-50 dark:bg-gray-700">
          <tr>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Strategy</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Symbol</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Venues</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Spread</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Profit</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Time</th>
          </tr>
        </thead>
        <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
          {opportunities.slice(0, 10).map((opp, idx) => (
            <tr key={idx}>
              <td className="px-4 py-3 whitespace-nowrap text-sm">{opp.strategy}</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm font-medium">{opp.symbol}</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm">{opp.venue_a} ↔ {opp.venue_b}</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm">{opp.spread_bps} bps</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm text-green-600 dark:text-green-400">${opp.estimated_profit.toFixed(2)}</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm text-gray-500">{new Date(opp.timestamp).toLocaleTimeString()}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
