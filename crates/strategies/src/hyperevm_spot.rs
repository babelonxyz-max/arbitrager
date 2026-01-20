use arb_core::types::{ArbitrageOpportunity, MarketData, PositionSide, SharedState, StrategyType, Venue};
use arb_core::RiskEngine;
use anyhow::Result;
use chrono::Utc;
use connectors::{HyperEvmConnector, HyperliquidConnector};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

pub struct HyperEvmSpotStrategy {
    hyperliquid: HyperliquidConnector,
    hyperevm: HyperEvmConnector,
    state: Arc<SharedState>,
    risk_engine: Arc<RiskEngine>,
    min_spread_bps: u64,
    check_interval: Duration,
    dry_run: bool,
}

impl HyperEvmSpotStrategy {
    pub fn new(
        hyperliquid: HyperliquidConnector,
        hyperevm: HyperEvmConnector,
        state: Arc<SharedState>,
        risk_engine: Arc<RiskEngine>,
        min_spread_bps: u64,
        check_interval_ms: u64,
        dry_run: bool,
    ) -> Self {
        Self {
            hyperliquid,
            hyperevm,
            state,
            risk_engine,
            min_spread_bps,
            check_interval: Duration::from_millis(check_interval_ms),
            dry_run,
        }
    }

    pub async fn run_loop(&self) {
        info!("Starting HyperEVM spot arbitrage loop");

        // Monitor a few key pairs
        let pairs = vec!["ETH-USDC", "BTC-USDC", "SOL-USDC"];

        loop {
            if self.risk_engine.is_kill_switch_active() {
                warn!("Kill switch active, pausing HyperEVM spot arb loop");
                sleep(Duration::from_secs(10)).await;
                continue;
            }

            for pair in &pairs {
                if let Err(e) = self.check_opportunity(pair).await {
                    error!("Error checking opportunity for {}: {}", pair, e);
                }
            }

            sleep(self.check_interval).await;
        }
    }

    async fn check_opportunity(&self, pair: &str) -> Result<()> {
        // Get spot price from HyperEVM
        let evm_price = self.hyperevm.get_spot_price(pair).await?;

        // Get reference price from Hyperliquid (using first token as symbol)
        let symbol = pair.split('-').next().unwrap_or("ETH");
        let hl_price = self.hyperliquid.get_market_data(symbol).await?;

        // Update state
        self.state.market_data.insert(
            (pair.to_string(), Venue::HyperEvm),
            evm_price.clone(),
        );
        self.state.market_data.insert(
            (symbol.to_string(), Venue::Hyperliquid),
            hl_price.clone(),
        );

        // Calculate spread
        let spread = if evm_price.price > hl_price.price {
            evm_price.price - hl_price.price
        } else {
            hl_price.price - evm_price.price
        };

        let spread_bps = (spread / hl_price.price * Decimal::from(10000u64))
            .to_u64()
            .unwrap_or(0);

        if spread_bps < self.min_spread_bps {
            return Ok(());
        }

        info!(
            "HyperEVM spot arb opportunity: {} - EVM: {}, HL: {}, spread: {} bps",
            pair,
            evm_price.price,
            hl_price.price,
            spread_bps
        );

        if self.dry_run {
            info!("DRY RUN: Would execute HyperEVM spot arbitrage");
            return Ok(());
        }

        // Execute arbitrage
        if evm_price.price > hl_price.price {
            // Buy on HL (cheaper), sell on EVM (more expensive)
            self.execute_arb(pair, symbol, &hl_price, &evm_price, PositionSide::Long, PositionSide::Short).await?;
        } else {
            // Buy on EVM (cheaper), sell on HL (more expensive)
            self.execute_arb(pair, symbol, &evm_price, &hl_price, PositionSide::Long, PositionSide::Short).await?;
        }

        Ok(())
    }

    async fn execute_arb(
        &self,
        pair: &str,
        symbol: &str,
        buy_market: &MarketData,
        sell_market: &MarketData,
        buy_side: PositionSide,
        sell_side: PositionSide,
    ) -> Result<()> {
        // Simplified execution - in production, would handle both legs atomically
        warn!("HyperEVM spot arbitrage execution not fully implemented");
        Ok(())
    }
}
