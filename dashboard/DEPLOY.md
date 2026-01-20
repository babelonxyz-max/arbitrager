# Deploy Dashboard to Vercel

## Quick Deploy

Run this command from the `dashboard` directory:

```bash
cd dashboard
npx vercel
```

Follow the prompts:
1. **Set up and deploy?** → Yes
2. **Which scope?** → Your Vercel account
3. **Link to existing project?** → No (first time) or Yes (if updating)
4. **Project name?** → arbitrager-dashboard (or your choice)
5. **Directory?** → `./` (current directory)
6. **Override settings?** → No

## Set Environment Variable

After deployment, go to your Vercel project dashboard:

1. Go to **Settings** → **Environment Variables**
2. Add: `NEXT_PUBLIC_BOT_API_URL`
3. Value: Your bot server URL (e.g., `https://your-bot.railway.app` or `http://localhost:8080` for local testing)
4. Click **Save**
5. Redeploy if needed

## Alternative: Deploy via GitHub

1. Push your code to GitHub
2. Go to [vercel.com](https://vercel.com)
3. Click **Add New Project**
4. Import your GitHub repository
5. Set **Root Directory** to `dashboard`
6. Add environment variable: `NEXT_PUBLIC_BOT_API_URL`
7. Click **Deploy**

## Verify Deployment

After deployment, visit your Vercel URL. You should see:
- Bot status card (may show "Connecting to bot..." if bot isn't running)
- Opportunities table
- Positions table
- Metrics chart

## Troubleshooting

**Dashboard shows "Connecting to bot..."**
- Ensure bot server is running and accessible
- Check `NEXT_PUBLIC_BOT_API_URL` is set correctly
- Verify bot API is accessible: `curl https://your-bot-url/api/status`

**Build fails**
- Check that all dependencies are in `package.json`
- Ensure TypeScript compiles: `npm run build`
