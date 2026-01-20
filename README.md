# Multi-Venue Arbitrage Bot

High-frequency arbitrage bot for Hyperliquid, HyperEVM, Binance, Bybit, and Solana (Jupiter).

## Features

- **Funding Arbitrage**: Compare funding rates across Hyperliquid, Binance, and Bybit
- **HyperEVM Spot Arbitrage**: Spot vs perpetual price discrepancies
- **Solana Jupiter Arbitrage**: Route-based arbitrage on Solana DEXs

## Architecture

Built with Rust using Tokio async runtime. Implements "Ralph loops" - tight, non-blocking execution loops for high-frequency trading.

## Setup

1. Copy `config/example.toml` to `config/local.toml` (gitignored)
2. Fill in API keys and RPC endpoints
3. Run: `cargo run --bin arb-daemon`

## Configuration

See `config/example.toml` for all configuration options.

**Security**: Never commit private keys. Use environment variables or `config/local.toml` (gitignored).

## Strategies

- `funding_arb`: Cross-exchange funding rate arbitrage
- `hyperevm_spot`: HyperEVM spot vs perp arbitrage
- `solana_jupiter`: Solana DEX route arbitrage

Enable/disable strategies in config file.
