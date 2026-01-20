#!/bin/bash

# Start Bot Server Script

echo "🚀 Starting Arbitrage Bot..."

# Check if config exists
if [ ! -f "config/local.toml" ]; then
    echo "⚠️  Config file not found. Creating from example..."
    cp config/example.toml config/local.toml
    echo "✅ Created config/local.toml"
    echo "📝 Note: Edit config/local.toml to add API keys (optional for dry-run mode)"
fi

# Check if port 8080 is in use
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null 2>&1 ; then
    echo "⚠️  Port 8080 is already in use!"
    echo "   Run: lsof -i :8080 to see what's using it"
    exit 1
fi

echo "🔨 Building bot..."
cargo build --bin arb-daemon

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Check errors above."
    exit 1
fi

echo "✅ Build successful!"
echo ""
echo "🌐 Starting bot server on http://localhost:8080"
echo "📊 Dashboard can connect to: http://localhost:8080"
echo ""
echo "Press Ctrl+C to stop"
echo ""

# Run the bot
cargo run --bin arb-daemon
