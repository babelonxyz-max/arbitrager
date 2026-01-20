#!/bin/bash

# Deploy Bot Script
# Usage: ./deploy-bot.sh [railway|render|fly]

set -e

PLATFORM=${1:-railway}

echo "🚀 Deploying bot to $PLATFORM..."

case $PLATFORM in
  railway)
    echo "📦 Deploying to Railway..."
    
    if ! command -v railway &> /dev/null; then
      echo "❌ Railway CLI not found. Install with: npm i -g @railway/cli"
      exit 1
    fi
    
    echo "🔐 Logging in to Railway..."
    railway login
    
    echo "⚙️  Setting up project..."
    if [ ! -f ".railway/project.json" ]; then
      railway init
    fi
    
    echo "📝 Setting environment variables..."
    railway variables set CONFIG_PATH=config/local.toml || true
    railway variables set RUST_LOG=info || true
    
    echo "🚀 Deploying..."
    railway up
    
    echo "✅ Deployment started!"
    echo ""
    echo "📋 Next steps:"
    echo "1. Get your bot URL: railway domain"
    echo "2. Set NEXT_PUBLIC_BOT_API_URL in Vercel dashboard"
    echo "3. Redeploy dashboard: cd dashboard && npx vercel --prod"
    ;;
    
  render)
    echo "📦 Deploying to Render..."
    echo ""
    echo "⚠️  Render deployment requires GitHub integration."
    echo "Please:"
    echo "1. Push this repo to GitHub"
    echo "2. Go to https://dashboard.render.com"
    echo "3. Create New Web Service"
    echo "4. Connect GitHub repo"
    echo "5. Use render.yaml config"
    echo ""
    echo "Or use Render CLI:"
    echo "  render deploy"
    ;;
    
  fly)
    echo "📦 Deploying to Fly.io..."
    
    if ! command -v fly &> /dev/null; then
      echo "❌ Fly CLI not found. Install with: curl -L https://fly.io/install.sh | sh"
      exit 1
    fi
    
    echo "🔐 Logging in to Fly.io..."
    fly auth login
    
    if [ ! -f "fly.toml" ]; then
      echo "⚙️  Initializing Fly.io app..."
      fly launch
    fi
    
    echo "🚀 Deploying..."
    fly deploy
    
    echo "✅ Deployment complete!"
    echo ""
    echo "📋 Your bot URL: https://arbitrager-bot.fly.dev"
    echo "Set NEXT_PUBLIC_BOT_API_URL in Vercel dashboard"
    ;;
    
  *)
    echo "❌ Unknown platform: $PLATFORM"
    echo "Usage: ./deploy-bot.sh [railway|render|fly]"
    exit 1
    ;;
esac
