# Arbitrage Bot Dashboard

Next.js dashboard for monitoring the arbitrage bot.

## Setup

1. Install dependencies:
```bash
cd dashboard
npm install
```

2. Set environment variables:
```bash
# .env.local
NEXT_PUBLIC_BOT_API_URL=http://localhost:8080
```

3. Run development server:
```bash
npm run dev
```

## Deployment to Vercel

1. Install Vercel CLI:
```bash
npm i -g vercel
```

2. Deploy:
```bash
vercel
```

3. Set environment variables in Vercel dashboard:
- `NEXT_PUBLIC_BOT_API_URL` - URL of your bot API server
- `BOT_API_URL` - Same as above (for server-side)

## Architecture

The dashboard connects to the bot's API server (running on port 8080) to display:
- Bot status and active strategies
- Recent arbitrage opportunities
- Active positions
- Metrics charts

The bot must be running separately (on a VPS, Railway, or similar) and exposing the API on port 8080.
