# Quick Deploy Guide

## Fastest Way: Railway (5 minutes)

### 1. Install Railway CLI
```bash
npm i -g @railway/cli
```

### 2. Deploy Bot
```bash
cd /Users/mark/arbitrager
railway login
railway init
railway up
```

### 3. Get Bot URL
```bash
railway domain
```
Copy the URL (e.g., `https://your-app.railway.app`)

### 4. Update Dashboard
```bash
cd dashboard
npx vercel env add NEXT_PUBLIC_BOT_API_URL production
# Paste your Railway URL when prompted
npx vercel --prod
```

**Done!** Your bot and dashboard are now live! 🎉

## Alternative: One-Command Deploy

```bash
./deploy-complete.sh
```

Follow the prompts.

## Manual Steps (If Scripts Don't Work)

### Bot Deployment

**Railway:**
1. Go to https://railway.app
2. New Project → Deploy from GitHub
3. Select your repo
4. Railway auto-detects Dockerfile
5. Set env vars in Railway dashboard
6. Deploy!

**Render:**
1. Go to https://render.com
2. New → Web Service
3. Connect GitHub repo
4. Use `render.yaml` config
5. Deploy!

### Dashboard Already Deployed

Your dashboard is already on Vercel:
- URL: https://dashboard-dp1igwy5i-marks-projects-95f7cc92.vercel.app

Just need to:
1. Deploy bot (above)
2. Set `NEXT_PUBLIC_BOT_API_URL` in Vercel
3. Redeploy dashboard

## Need Help?

See `DEPLOY_ALL.md` for detailed instructions for all platforms.
