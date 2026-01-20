# Quick Fix: Dashboard Not Connecting

## The Problem

The dashboard isn't connecting because **the bot server is not running**.

## Solution (3 Steps)

### Step 1: Start the Bot Server

```bash
cd /Users/mark/arbitrager
./start-bot.sh
```

Or manually:
```bash
cd /Users/mark/arbitrager
cargo run --bin arb-daemon
```

You should see:
```
Starting API server on http://0.0.0.0:8080
All strategies started. API server running on port 8080.
```

### Step 2: Verify API is Working

In another terminal:
```bash
curl http://localhost:8080/api/status
```

Should return JSON like:
```json
{"status":"running","strategies":{...},"kill_switch_active":false,...}
```

### Step 3: Update Dashboard Environment Variable

**For Local Testing:**
```bash
cd dashboard
echo "NEXT_PUBLIC_BOT_API_URL=http://localhost:8080" > .env.local
npm run dev
```

**For Vercel (Production):**
1. Go to: https://vercel.com/marks-projects-95f7cc92/dashboard/settings/environment-variables
2. Add: `NEXT_PUBLIC_BOT_API_URL`
3. Value: `http://localhost:8080` (for local bot) or your production bot URL
4. Redeploy: `cd dashboard && npx vercel --prod`

## Why This Happens

The dashboard (on Vercel) tries to connect to the bot API server. If the bot isn't running, the connection fails.

**Architecture:**
```
Dashboard (Vercel) → HTTP Request → Bot Server (Port 8080)
```

Both need to be running and accessible to each other.

## Common Issues

1. **Bot not running**: Start it with `./start-bot.sh`
2. **Wrong URL**: Check `NEXT_PUBLIC_BOT_API_URL` matches bot location
3. **Port blocked**: Ensure port 8080 is accessible
4. **Config missing**: Bot creates `config/local.toml` automatically

## Test Connection

```bash
# Test bot API
curl http://localhost:8080/api/status

# Test from dashboard location
# (If dashboard is on Vercel, bot needs public URL)
```

## Next Steps

Once bot is running:
1. ✅ Bot API responds to `curl http://localhost:8080/api/status`
2. ✅ Dashboard env var points to correct URL
3. ✅ Dashboard refreshes and shows bot status

The dashboard auto-refreshes every 5 seconds once connected!
