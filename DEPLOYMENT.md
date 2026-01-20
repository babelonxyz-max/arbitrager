# Deployment Guide

## Overview

This project consists of two components:
1. **Rust Bot** (`arb-daemon`) - The arbitrage trading bot
2. **Next.js Dashboard** - Web interface for monitoring

## Architecture

```
┌─────────────────┐         ┌──────────────┐
│  Vercel         │         │  Bot Server  │
│  (Dashboard)    │────────▶│  (VPS/Railway)│
│  Next.js        │  HTTP   │  Port 8080   │
└─────────────────┘         └──────────────┘
```

## Deploying the Dashboard to Vercel

### Option 1: Vercel CLI

```bash
cd dashboard
vercel
```

### Option 2: GitHub Integration

1. Push your code to GitHub
2. Go to [vercel.com](https://vercel.com)
3. Import your repository
4. Set root directory to `dashboard`
5. Add environment variable: `NEXT_PUBLIC_BOT_API_URL`

## Running the Bot

The bot needs to run continuously. Options:

### Option 1: VPS (Recommended)

```bash
# On your VPS
cd /path/to/arbitrager
cargo build --release --bin arb-daemon
./target/release/arb-daemon
```

### Option 2: Railway

1. Create `Railway.toml`:
```toml
[build]
builder = "NIXPACKS"

[deploy]
startCommand = "cargo run --release --bin arb-daemon"
```

2. Deploy via Railway CLI or GitHub integration

### Option 3: Docker

Create `Dockerfile`:
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin arb-daemon

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/arb-daemon /usr/local/bin/arb-daemon
EXPOSE 8080
CMD ["arb-daemon"]
```

## Environment Variables

### Bot Server
- `CONFIG_PATH` - Path to config file (default: `config/local.toml`)
- `HYPERLIQUID_PRIVATE_KEY` - Hyperliquid API key
- `BINANCE_API_KEY` - Binance API key
- `BINANCE_API_SECRET` - Binance API secret
- `BYBIT_API_KEY` - Bybit API key
- `BYBIT_API_SECRET` - Bybit API secret
- `SOLANA_PRIVATE_KEY` - Solana wallet private key

### Dashboard (Vercel)
- `NEXT_PUBLIC_BOT_API_URL` - URL of bot API server (e.g., `https://your-bot.railway.app`)

## Security Notes

1. Never commit private keys or secrets
2. Use Vercel environment variables for dashboard config
3. Use secure storage (Vault, AWS Secrets Manager) for bot credentials
4. Enable HTTPS for bot API server
5. Consider adding authentication to bot API endpoints

## Monitoring

The dashboard automatically refreshes every 5 seconds. It displays:
- Bot status (running/stopped/error)
- Active strategies
- Recent opportunities
- Active positions
- Metrics charts

## Troubleshooting

### Dashboard can't connect to bot
- Check `NEXT_PUBLIC_BOT_API_URL` is correct
- Verify bot is running and accessible
- Check firewall/security groups allow port 8080

### Bot not starting
- Check config file exists at `config/local.toml`
- Verify all required API keys are set
- Check logs for specific errors

### CORS errors
- Bot API server includes CORS middleware
- Ensure bot is accessible from dashboard domain
