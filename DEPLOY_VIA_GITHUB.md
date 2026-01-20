# 🚀 Deploy via GitHub (Easiest Method)

Since Railway CLI requires interactive login, here's the easiest way to deploy:

## Step 1: Push to GitHub

```bash
cd /Users/mark/arbitrager

# Create a new repo on GitHub first, then:
git remote add origin https://github.com/YOUR_USERNAME/arbitrager.git
git branch -M main
git push -u origin main
```

Or use GitHub CLI:
```bash
gh repo create arbitrager --public --source=. --remote=origin --push
```

## Step 2: Deploy Bot to Render (Easiest)

1. **Go to**: https://dashboard.render.com
2. **Click**: "New +" → "Web Service"
3. **Connect**: Your GitHub account
4. **Select**: Your `arbitrager` repository
5. **Configure**:
   - Name: `arbitrager-bot`
   - Environment: `Docker`
   - Render will auto-detect `render.yaml`
6. **Set Environment Variables**:
   - `CONFIG_PATH` = `config/local.toml`
   - `RUST_LOG` = `info`
   - (Add API keys if needed)
7. **Click**: "Create Web Service"
8. **Wait**: ~5-10 minutes for build
9. **Get URL**: Render provides URL like `https://arbitrager-bot.onrender.com`

## Step 3: Connect Dashboard

```bash
cd dashboard
npx vercel env add NEXT_PUBLIC_BOT_API_URL production
# Paste your Render URL when prompted
npx vercel --prod
```

## Alternative: Railway via GitHub

1. **Go to**: https://railway.app
2. **Click**: "New Project"
3. **Select**: "Deploy from GitHub repo"
4. **Choose**: Your `arbitrager` repo
5. **Railway auto-detects**: Dockerfile
6. **Set Environment Variables** in Railway dashboard
7. **Deploy!**

## Quick Commands

```bash
# 1. Push to GitHub (if not done)
git remote add origin https://github.com/YOUR_USERNAME/arbitrager.git
git push -u origin main

# 2. Deploy bot on Render (via web UI)
# Go to render.com and follow steps above

# 3. Update dashboard
cd dashboard
npx vercel env add NEXT_PUBLIC_BOT_API_URL production
npx vercel --prod
```

## That's It! 🎉

Your bot will be live in ~10 minutes!
