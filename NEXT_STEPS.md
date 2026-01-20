# ✅ Ready to Deploy! Next Steps

## ✅ What's Done

- ✅ Git repository initialized
- ✅ All code committed
- ✅ Dockerfile created
- ✅ Deployment configs ready
- ✅ Dashboard already on Vercel

## 🚀 Deploy Now (3 Steps)

### Step 1: Push to GitHub

**Create a new repository on GitHub first:**
1. Go to https://github.com/new
2. Repository name: `arbitrager`
3. Choose Public or Private
4. **Don't** initialize with README
5. Click "Create repository"

**Then push your code:**
```bash
cd /Users/mark/arbitrager
git remote add origin https://github.com/YOUR_USERNAME/arbitrager.git
git branch -M main
git push -u origin main
```

Replace `YOUR_USERNAME` with your GitHub username.

### Step 2: Deploy Bot to Render

1. **Go to**: https://dashboard.render.com
2. **Click**: "New +" → "Web Service"
3. **Connect**: Your GitHub account (if not connected)
4. **Select**: Your `arbitrager` repository
5. **Configure**:
   - **Name**: `arbitrager-bot`
   - **Environment**: `Docker`
   - Render will auto-detect the Dockerfile
6. **Environment Variables** (click "Advanced"):
   - `CONFIG_PATH` = `config/local.toml`
   - `RUST_LOG` = `info`
   - (Add API keys later if needed - not required for dry-run mode)
7. **Click**: "Create Web Service"
8. **Wait**: Build takes ~5-10 minutes
9. **Copy URL**: You'll get something like `https://arbitrager-bot.onrender.com`

### Step 3: Connect Dashboard

```bash
cd dashboard
npx vercel env add NEXT_PUBLIC_BOT_API_URL production
```

When prompted, paste your Render URL (e.g., `https://arbitrager-bot.onrender.com`)

Then redeploy:
```bash
npx vercel --prod
```

## ✅ Verify

1. **Test bot API**:
```bash
curl https://your-render-url.onrender.com/api/status
```

Should return JSON with bot status.

2. **Check dashboard**:
Visit: https://dashboard-dp1igwy5i-marks-projects-95f7cc92.vercel.app

Should show bot status (not "Connecting to bot...").

## 🎉 Done!

Your arbitrage bot is now:
- ✅ Running 24/7 on Render
- ✅ Monitoring opportunities
- ✅ Dashboard showing real-time status
- ✅ Ready to trade (in dry-run mode by default)

## 📋 Quick Copy-Paste

```bash
# 1. Add GitHub remote (replace YOUR_USERNAME)
git remote add origin https://github.com/YOUR_USERNAME/arbitrager.git
git push -u origin main

# 2. Deploy on Render (via web UI at render.com)

# 3. Connect dashboard
cd dashboard
npx vercel env add NEXT_PUBLIC_BOT_API_URL production
npx vercel --prod
```

## Need Help?

- See `DEPLOY_INSTRUCTIONS.md` for detailed steps
- See `TROUBLESHOOTING.md` for common issues
- Check Render logs if bot won't start
