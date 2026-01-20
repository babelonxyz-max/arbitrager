'use client'

interface BotStatus {
  status: 'running' | 'stopped' | 'error'
  strategies: {
    funding_arb: boolean
    hyperevm_spot: boolean
    solana_jupiter: boolean
  }
  kill_switch_active: boolean
  dry_run: boolean
}

export default function StatusCard({ status }: { status: BotStatus | null }) {
  if (!status) {
    return (
      <div className="bg-yellow-100 dark:bg-yellow-900 rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold mb-2">Status</h3>
        <p className="text-gray-600 dark:text-gray-300">Connecting to bot...</p>
      </div>
    )
  }

  const statusColors = {
    running: 'bg-green-100 dark:bg-green-900',
    stopped: 'bg-gray-100 dark:bg-gray-800',
    error: 'bg-red-100 dark:bg-red-900',
  }

  const activeStrategies = Object.values(status.strategies).filter(Boolean).length

  return (
    <>
      <div className={`${statusColors[status.status]} rounded-lg shadow p-6`}>
        <h3 className="text-lg font-semibold mb-2">Bot Status</h3>
        <p className="text-2xl font-bold capitalize">{status.status}</p>
        {status.dry_run && (
          <p className="text-sm mt-2 text-yellow-700 dark:text-yellow-300">DRY RUN MODE</p>
        )}
        {status.kill_switch_active && (
          <p className="text-sm mt-2 text-red-700 dark:text-red-300">⚠️ KILL SWITCH ACTIVE</p>
        )}
      </div>

      <div className="bg-blue-100 dark:bg-blue-900 rounded-lg shadow p-6">
        <h3 className="text-lg font-semibold mb-2">Active Strategies</h3>
        <p className="text-2xl font-bold">{activeStrategies} / 3</p>
        <div className="mt-2 text-sm">
          {status.strategies.funding_arb && <div>✓ Funding Arb</div>}
          {status.strategies.hyperevm_spot && <div>✓ HyperEVM Spot</div>}
          {status.strategies.solana_jupiter && <div>✓ Solana Jupiter</div>}
        </div>
      </div>
    </>
  )
}
