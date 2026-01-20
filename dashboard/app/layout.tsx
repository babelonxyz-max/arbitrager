import './globals.css'
import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'Arbitrage Bot Dashboard',
  description: 'Monitor multi-venue arbitrage opportunities',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
