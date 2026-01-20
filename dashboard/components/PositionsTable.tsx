'use client'

interface Position {
  symbol: string
  venue: string
  side: string
  size: number
  entry_price: number
  leverage: number
}

export default function PositionsTable({ positions }: { positions: Position[] }) {
  if (positions.length === 0) {
    return <p className="text-gray-500">No active positions.</p>
  }

  return (
    <div className="overflow-x-auto">
      <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
        <thead className="bg-gray-50 dark:bg-gray-700">
          <tr>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Symbol</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Venue</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Side</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Size</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Entry</th>
            <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">Leverage</th>
          </tr>
        </thead>
        <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
          {positions.map((pos, idx) => (
            <tr key={idx}>
              <td className="px-4 py-3 whitespace-nowrap text-sm font-medium">{pos.symbol}</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm">{pos.venue}</td>
              <td className={`px-4 py-3 whitespace-nowrap text-sm ${
                pos.side === 'Long' ? 'text-green-600' : 'text-red-600'
              }`}>{pos.side}</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm">{pos.size.toFixed(4)}</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm">${pos.entry_price.toFixed(2)}</td>
              <td className="px-4 py-3 whitespace-nowrap text-sm">{pos.leverage}x</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
