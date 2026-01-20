'use client'

import { useEffect, useState } from 'react'
import StatusCard from '@/components/StatusCard'
import OpportunitiesTable from '@/components/OpportunitiesTable'
import PositionsTable from '@/components/PositionsTable'
import MetricsChart from '@/components/MetricsChart'

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

interface Opportunity {
  strategy: string
  symbol: string
  venue_a: string
  venue_b: string
  spread_bps: number
  estimated_profit: number
  timestamp: string
}

interface Position {
  symbol: string
  venue: string
  side: string
  size: number
  entry_price: number
  leverage: number
}

export default function Home() {
  const [status, setStatus] = useState<BotStatus | null>(null)
  const [opportunities, setOpportunities] = useState<Opportunity[]>([])
  const [positions, setPositions] = useState<Position[]>([])
  const [loading, setLoading] = useState(true)

  const fetchData = async () => {
    try {
      const apiUrl = process.env.NEXT_PUBLIC_BOT_API_URL || 'http://localhost:8080'
      
      const [statusRes, oppsRes, posRes] = await Promise.all([
        fetch(`${apiUrl}/api/status`).catch(() => null),
        fetch(`${apiUrl}/api/opportunities`).catch(() => null),
        fetch(`${apiUrl}/api/positions`).catch(() => null),
      ])

      if (statusRes?.ok) {
        const statusData = await statusRes.json()
        setStatus(statusData)
      }

      if (oppsRes?.ok) {
        const oppsData = await oppsRes.json()
        setOpportunities(oppsData.opportunities || [])
      }

      if (posRes?.ok) {
        const posData = await posRes.json()
        setPositions(posData.positions || [])
      }
    } catch (error) {
      console.error('Failed to fetch data:', error)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchData()
    const interval = setInterval(fetchData, 5000) // Refresh every 5 seconds
    return () => clearInterval(interval)
  }, [])

  if (loading && !status) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-xl">Loading...</div>
      </div>
    )
  }

  return (
    <main className="min-h-screen p-8">
      <div className="max-w-7xl mx-auto">
        <h1 className="text-4xl font-bold mb-8">Arbitrage Bot Dashboard</h1>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
          <StatusCard status={status} />
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
            <h2 className="text-2xl font-semibold mb-4">Recent Opportunities</h2>
            <OpportunitiesTable opportunities={opportunities} />
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
            <h2 className="text-2xl font-semibold mb-4">Active Positions</h2>
            <PositionsTable positions={positions} />
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h2 className="text-2xl font-semibold mb-4">Metrics</h2>
          <MetricsChart opportunities={opportunities} />
        </div>
      </div>
    </main>
  )
}
