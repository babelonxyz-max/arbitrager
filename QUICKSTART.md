# Quick Start Guide

## Deploy Dashboard to Vercel

1. **Install Vercel CLI** (if not already installed):
```bash
npm i -g vercel
```

2. **Navigate to dashboard directory**:
```bash
cd dashboard
```

3. **Deploy to Vercel**:
```bash
vercel
```

4. **Set environment variable** in Vercel dashboard:
   - Go to your project settings → Environment Variables
   - Add: `NEXT_PUBLIC_BOT_API_URL` = `https://your-bot-server.com` (or `http://localhost:8080` for local testing)

## Run Bot Locally (for testing)

1. **Build the bot**:
```bash
cargo build --release --bin arb-daemon
```

2. **Create config file**:
```bash
cp config/example.toml config/local.toml
# Edit config/local.toml with your API keys
```

3. **Run the bot**:
```bash
./target/release/arb-daemon
```

The bot will:
- Start all enabled strategies
- Run API server on `http://localhost:8080`
- Dashboard can connect to it

## Run Dashboard Locally

1. **Install dependencies**:
```bash
cd dashboard
npm install
```

2. **Set environment variable**:
```bash
echo "NEXT_PUBLIC_BOT_API_URL=http://localhost:8080" > .env.local
```

3. **Run dev server**:
```bash
npm run dev
```

4. **Open browser**: `http://localhost:3000`

## Architecture

```
┌─────────────────────┐
│   Vercel (Dashboard)│
│   Next.js App       │
│   Port 3000         │
└──────────┬──────────┘
           │ HTTP
           │
┌──────────▼──────────┐
│   Bot Server        │
│   Rust + Axum       │
│   Port 8080         │
└─────────────────────┘
```

The dashboard fetches data from the bot's API every 5 seconds.

## Production Deployment

### Bot Server Options:

1. **VPS** (Recommended for production)
   - Deploy bot binary
   - Use systemd/PM2 to keep it running
   - Expose port 8080

2. **Railway**
   - Connect GitHub repo
   - Set build command: `cargo build --release --bin arb-daemon`
   - Set start command: `./target/release/arb-daemon`
   - Railway will provide HTTPS URL

3. **Docker**
   - Build: `docker build -t arb-bot .`
   - Run: `docker run -p 8080:8080 arb-bot`

### Dashboard on Vercel:
- Already configured via `vercel.json`
- Just run `vercel` from dashboard directory
- Set `NEXT_PUBLIC_BOT_API_URL` to your bot's public URL

## Troubleshooting

**Dashboard shows "Connecting to bot..."**
- Check bot is running: `curl http://localhost:8080/api/status`
- Verify `NEXT_PUBLIC_BOT_API_URL` is correct
- Check CORS settings (bot includes CORS middleware)

**Bot won't start**
- Check config file exists: `config/local.toml`
- Verify API keys are set
- Check logs for specific errors

**CORS errors**
- Bot API includes CORS middleware
- Ensure bot URL is accessible from dashboard domain
