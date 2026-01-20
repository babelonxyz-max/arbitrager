# 🚀 Deploy Everything Now

## ✅ What's Ready

1. **Dashboard**: ✅ Already deployed on Vercel
   - URL: https://dashboard-dp1igwy5i-marks-projects-95f7cc92.vercel.app
   - Status: Live, waiting for bot

2. **Bot Server**: ✅ Ready to deploy
   - Dockerfile ✅
   - Config files ✅
   - API server ✅

## 🎯 Deploy Bot (Choose One Method)

### Method 1: Railway (Recommended - Easiest)

**Step 1: Install Railway CLI**
```bash
npx @railway/cli login
```

**Step 2: Initialize & Deploy**
```bash
cd /Users/mark/arbitrager
npx @railway/cli init
npx @railway/cli up
```

**Step 3: Get URL**
```bash
npx @railway/cli domain
```

**Step 4: Set Environment Variables** (in Railway dashboard)
- Go to your Railway project → Variables
- Add: `CONFIG_PATH` = `config/local.toml`
- Add: `RUST_LOG` = `info`
- Add your API keys (optional for dry-run)

### Method 2: Render (Via Web UI)

1. **Push to GitHub** (if not already):
```bash
cd /Users/mark/arbitrager
git init
git add .
git commit -m "Initial commit"
# Push to GitHub (create repo first)
```

2. **Deploy on Render**:
   - Go to https://dashboard.render.com
   - New → Web Service
   - Connect GitHub repo
   - Render will auto-detect `render.yaml`
   - Set environment variables
   - Deploy!

### Method 3: Fly.io

```bash
# Install Fly CLI
curl -L https://fly.io/install.sh | sh

# Login
fly auth login

# Deploy
cd /Users/mark/arbitrager
fly launch
fly deploy
```

## 🔗 Connect Dashboard to Bot

After bot is deployed:

1. **Get your bot URL** (from Railway/Render/Fly)

2. **Update Vercel environment variable**:
```bash
cd dashboard
npx vercel env add NEXT_PUBLIC_BOT_API_URL production
# When prompted, paste your bot URL
```

3. **Redeploy dashboard**:
```bash
npx vercel --prod
```

## ✅ Verify Everything Works

1. **Test bot API**:
```bash
curl https://your-bot-url/api/status
```

Should return:
```json
{"status":"running","strategies":{...},"kill_switch_active":false}
```

2. **Check dashboard**:
   - Visit: https://dashboard-dp1igwy5i-marks-projects-95f7cc92.vercel.app
   - Should show bot status (not "Connecting to bot...")

## 📋 Quick Checklist

- [ ] Deploy bot to Railway/Render/Fly
- [ ] Get bot URL
- [ ] Set `NEXT_PUBLIC_BOT_API_URL` in Vercel
- [ ] Redeploy dashboard
- [ ] Test bot API: `curl https://your-bot-url/api/status`
- [ ] Verify dashboard shows bot status

## 🆘 Need Help?

- **Detailed guide**: See `DEPLOY_ALL.md`
- **Troubleshooting**: See `TROUBLESHOOTING.md`
- **Quick fix**: See `QUICK_FIX.md`

## 🎉 You're Done!

Once both are deployed and connected, your arbitrage bot will be:
- ✅ Running 24/7
- ✅ Monitoring opportunities
- ✅ Dashboard showing real-time status
- ✅ Ready to trade (when you enable live mode)
