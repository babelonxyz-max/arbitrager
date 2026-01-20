#!/bin/bash

# Complete Deployment Script
# Deploys both bot and dashboard

set -e

echo "🚀 Complete Deployment Script"
echo "=============================="
echo ""

# Step 1: Deploy Bot
echo "📦 Step 1: Deploying Bot Server..."
read -p "Choose platform [railway/render/fly]: " PLATFORM

if [ -z "$PLATFORM" ]; then
  PLATFORM="railway"
fi

./deploy-bot.sh $PLATFORM

echo ""
echo "⏳ Waiting for bot deployment..."
sleep 5

# Step 2: Get Bot URL
echo ""
echo "📋 Step 2: Get Bot URL"
echo "Please provide your bot URL (e.g., https://your-app.railway.app):"
read BOT_URL

if [ -z "$BOT_URL" ]; then
  echo "⚠️  No bot URL provided. Skipping dashboard update."
  exit 0
fi

# Step 3: Update Dashboard
echo ""
echo "🌐 Step 3: Updating Dashboard on Vercel..."

cd dashboard

# Check if vercel is linked
if [ ! -f ".vercel/project.json" ]; then
  echo "⚠️  Vercel project not linked. Linking now..."
  npx vercel link
fi

echo "📝 Setting environment variable..."
npx vercel env add NEXT_PUBLIC_BOT_API_URL production <<< "$BOT_URL"

echo "🚀 Redeploying dashboard..."
npx vercel --prod

echo ""
echo "✅ Deployment Complete!"
echo ""
echo "📊 Dashboard: https://dashboard-dp1igwy5i-marks-projects-95f7cc92.vercel.app"
echo "🤖 Bot API: $BOT_URL"
echo ""
echo "🔍 Verify:"
echo "  curl $BOT_URL/api/status"
echo ""
