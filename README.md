# Multi-Venue Arbitrage Bot

High-frequency arbitrage bot for Hyperliquid, HyperEVM, Binance, Bybit, and Solana (Jupiter).

## Features

- **Funding Arbitrage**: Compare funding rates across Hyperliquid, Binance, and Bybit
- **HyperEVM Spot Arbitrage**: Spot vs perpetual price discrepancies
- **Solana Jupiter Arbitrage**: Route-based arbitrage on Solana DEXs
- **Real-time Dashboard**: Monitor bot status, opportunities, and positions

## Quick Start

### Local Development

```bash
# Start bot
./start-bot.sh

# Start dashboard (in another terminal)
cd dashboard
npm install
npm run dev
```

## Deployment

See `DEPLOY_NOW.md` for deployment instructions.

## Architecture

- **Bot Server**: Rust + Tokio + Axum (port 8080)
- **Dashboard**: Next.js + React (Vercel)
- **Strategies**: Funding arb, HyperEVM spot, Solana Jupiter

## Configuration

Copy `config/example.toml` to `config/local.toml` and fill in API keys.

## License

MIT
