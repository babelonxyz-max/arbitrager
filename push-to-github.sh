#!/bin/bash

# Push to GitHub script
# This will push all code to https://github.com/babeloxyz/solarb

set -e

echo "🚀 Pushing code to GitHub..."

# Ensure remote is correct
git remote set-url origin https://github.com/babeloxyz/solarb.git

# Check if repo exists
echo "📦 Checking repository..."
if ! git ls-remote origin &>/dev/null; then
    echo "⚠️  Repository not found or not accessible"
    echo "Please ensure:"
    echo "1. Repository 'solarb' exists at https://github.com/babeloxyz/solarb"
    echo "2. You have push access"
    exit 1
fi

# Push code
echo "📤 Pushing commits..."
git push -u origin main

echo "✅ Successfully pushed to GitHub!"
echo ""
echo "🔗 Repository: https://github.com/babeloxyz/solarb"
echo ""
echo "Next steps:"
echo "1. Deploy bot to Render: https://dashboard.render.com"
echo "2. Connect dashboard: cd dashboard && npx vercel env add NEXT_PUBLIC_BOT_API_URL production"
