# Deploy Everything: Bot + Dashboard

This guide will help you deploy both the bot server and dashboard to production.

## Architecture

```
┌─────────────────┐         ┌──────────────────┐
│  Vercel         │────────▶│  Railway/Render  │
│  (Dashboard)    │  HTTP   │  (Bot Server)    │
│  Next.js        │         │  Rust + Axum     │
└─────────────────┘         └──────────────────┘
```

## Option 1: Railway (Recommended - Easiest)

### Deploy Bot to Railway

1. **Install Railway CLI**:
```bash
npm i -g @railway/cli
```

2. **Login to Railway**:
```bash
railway login
```

3. **Initialize Railway project**:
```bash
cd /Users/mark/arbitrager
railway init
```

4. **Set environment variables**:
```bash
# Set config path
railway variables set CONFIG_PATH=config/local.toml

# Set your API keys (optional for dry-run mode)
railway variables set HYPERLIQUID_PRIVATE_KEY=your_key
railway variables set BINANCE_API_KEY=your_key
railway variables set BINANCE_API_SECRET=your_secret
railway variables set BYBIT_API_KEY=your_key
railway variables set BYBIT_API_SECRET=your_secret
railway variables set SOLANA_PRIVATE_KEY=your_key

# Set log level
railway variables set RUST_LOG=info
```

5. **Deploy**:
```bash
railway up
```

6. **Get your bot URL**:
```bash
railway domain
```
This will give you a URL like: `https://your-app.railway.app`

### Update Dashboard on Vercel

1. **Set environment variable**:
   - Go to: https://vercel.com/marks-projects-95f7cc92/dashboard/settings/environment-variables
   - Add: `NEXT_PUBLIC_BOT_API_URL`
   - Value: Your Railway URL (e.g., `https://your-app.railway.app`)
   - Click **Save**

2. **Redeploy dashboard**:
```bash
cd dashboard
npx vercel --prod
```

## Option 2: Render

### Deploy Bot to Render

1. **Go to Render Dashboard**: https://dashboard.render.com

2. **Create New Web Service**:
   - Connect your GitHub repository
   - Select the repository
   - Set:
     - **Name**: `arbitrager-bot`
     - **Environment**: `Docker`
     - **Dockerfile Path**: `Dockerfile`
     - **Docker Context**: `.`
     - **Start Command**: `arb-daemon`

3. **Set Environment Variables**:
   - `CONFIG_PATH` = `config/local.toml`
   - `RUST_LOG` = `info`
   - Add your API keys (HYPERLIQUID_PRIVATE_KEY, etc.)

4. **Deploy**: Click "Create Web Service"

5. **Get URL**: Render will provide a URL like `https://arbitrager-bot.onrender.com`

### Update Dashboard

Same as Railway - set `NEXT_PUBLIC_BOT_API_URL` in Vercel to your Render URL.

## Option 3: Fly.io

### Deploy Bot to Fly.io

1. **Install Fly CLI**:
```bash
curl -L https://fly.io/install.sh | sh
```

2. **Login**:
```bash
fly auth login
```

3. **Launch**:
```bash
cd /Users/mark/arbitrager
fly launch
```

4. **Set secrets**:
```bash
fly secrets set HYPERLIQUID_PRIVATE_KEY=your_key
fly secrets set BINANCE_API_KEY=your_key
# ... etc
```

5. **Deploy**:
```bash
fly deploy
```

6. **Get URL**: `https://arbitrager-bot.fly.dev`

## Option 4: Docker + Any Platform

### Build Docker Image

```bash
cd /Users/mark/arbitrager
docker build -t arbitrager-bot .
```

### Run Locally

```bash
docker run -p 8080:8080 \
  -e CONFIG_PATH=config/local.toml \
  -e HYPERLIQUID_PRIVATE_KEY=your_key \
  arbitrager-bot
```

### Push to Registry

```bash
# Tag for your registry
docker tag arbitrager-bot your-registry/arbitrager-bot

# Push
docker push your-registry/arbitrager-bot
```

Then deploy to:
- AWS ECS/Fargate
- Google Cloud Run
- Azure Container Instances
- DigitalOcean App Platform
- Any Docker-compatible platform

## Quick Deploy Script

I've created a helper script:

```bash
./deploy-bot.sh railway
# or
./deploy-bot.sh render
# or
./deploy-bot.sh fly
```

## Environment Variables Reference

### Required (for bot to start)
- `CONFIG_PATH` - Path to config (default: `config/local.toml`)

### Optional (for trading - can use defaults for dry-run)
- `HYPERLIQUID_PRIVATE_KEY` - Hyperliquid API key
- `BINANCE_API_KEY` - Binance API key
- `BINANCE_API_SECRET` - Binance API secret
- `BYBIT_API_KEY` - Bybit API key
- `BYBIT_API_SECRET` - Bybit API secret
- `SOLANA_PRIVATE_KEY` - Solana wallet private key
- `RUST_LOG` - Log level (default: `info`)

### Dashboard (Vercel)
- `NEXT_PUBLIC_BOT_API_URL` - Your bot server URL

## Verify Deployment

1. **Check bot is running**:
```bash
curl https://your-bot-url.com/api/status
```

Should return:
```json
{
  "status": "running",
  "strategies": {...},
  "kill_switch_active": false,
  "dry_run": true
}
```

2. **Check dashboard**:
   - Visit your Vercel URL
   - Should show bot status (not "Connecting to bot...")

## Troubleshooting

### Bot won't start
- Check logs: `railway logs` or Render/Fly dashboard
- Verify config file exists
- Check environment variables are set

### Dashboard can't connect
- Verify bot URL is correct
- Check bot is accessible: `curl https://your-bot-url/api/status`
- Ensure CORS is enabled (bot includes CORS middleware)
- Check firewall/security groups allow port 8080

### Build fails
- Ensure Dockerfile is correct
- Check Rust version compatibility
- Verify all dependencies are in Cargo.toml

## Next Steps

1. ✅ Deploy bot to Railway/Render/Fly
2. ✅ Get bot URL
3. ✅ Set `NEXT_PUBLIC_BOT_API_URL` in Vercel
4. ✅ Redeploy dashboard
5. ✅ Monitor both services

Your arbitrage bot is now live! 🚀
