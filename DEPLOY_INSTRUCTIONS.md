# 🚀 Deployment Instructions

## Current Status

✅ **Dashboard**: Already deployed on Vercel
- URL: https://dashboard-dp1igwy5i-marks-projects-95f7cc92.vercel.app

✅ **Bot**: Code ready, needs deployment

## Fastest Deployment Path

### Option 1: Render (Recommended - No CLI needed)

1. **Push to GitHub**:
```bash
cd /Users/mark/arbitrager
git remote add origin https://github.com/YOUR_USERNAME/arbitrager.git
git push -u origin main
```

2. **Deploy on Render**:
   - Visit: https://dashboard.render.com
   - Click "New +" → "Web Service"
   - Connect GitHub → Select `arbitrager` repo
   - Render auto-detects Dockerfile
   - Set env vars: `CONFIG_PATH=config/local.toml`, `RUST_LOG=info`
   - Click "Create Web Service"
   - Wait ~5-10 minutes
   - Copy your URL (e.g., `https://arbitrager-bot.onrender.com`)

3. **Connect Dashboard**:
```bash
cd dashboard
npx vercel env add NEXT_PUBLIC_BOT_API_URL production
# Paste Render URL when prompted
npx vercel --prod
```

**Done!** 🎉

### Option 2: Railway (Via GitHub)

1. Push to GitHub (same as above)

2. Deploy on Railway:
   - Visit: https://railway.app
   - "New Project" → "Deploy from GitHub repo"
   - Select your repo
   - Railway auto-detects Dockerfile
   - Set environment variables
   - Deploy!

### Option 3: Manual Docker Build

If you have Docker installed:

```bash
# Build
docker build -t arbitrager-bot .

# Run locally to test
docker run -p 8080:8080 \
  -e CONFIG_PATH=config/local.toml \
  arbitrager-bot

# Push to registry and deploy to your platform
```

## What Happens Next

1. Bot builds (~5-10 minutes)
2. Bot starts and exposes API on port 8080
3. Dashboard connects and shows status
4. Bot monitors for arbitrage opportunities
5. Dashboard updates every 5 seconds

## Verify Deployment

```bash
# Test bot API
curl https://your-bot-url/api/status

# Should return:
# {"status":"running","strategies":{...},"kill_switch_active":false}
```

## Need Help?

- See `DEPLOY_VIA_GITHUB.md` for detailed GitHub steps
- See `TROUBLESHOOTING.md` for common issues
- Check platform logs if bot won't start

## Quick Start (Copy-Paste)

```bash
# 1. Initialize git (if needed)
cd /Users/mark/arbitrager
git init
git add .
git commit -m "Initial commit"

# 2. Create GitHub repo and push
# (Do this on github.com first, then:)
git remote add origin https://github.com/YOUR_USERNAME/arbitrager.git
git push -u origin main

# 3. Deploy on Render (web UI)
# Go to render.com → New Web Service → Connect repo

# 4. Connect dashboard
cd dashboard
npx vercel env add NEXT_PUBLIC_BOT_API_URL production
npx vercel --prod
```

Your arbitrage bot will be live! 🚀
