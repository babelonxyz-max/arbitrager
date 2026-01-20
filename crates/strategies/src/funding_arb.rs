use arb_core::types::{ArbitrageOpportunity, FundingRate, PositionSide, SharedState, StrategyType, Trade, TradeStatus, Venue};
use arb_core::RiskEngine;
use anyhow::Result;
use chrono::Utc;
use connectors::{BinanceConnector, BybitConnector, HyperliquidConnector};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

pub struct FundingArbStrategy {
    hyperliquid: HyperliquidConnector,
    binance: BinanceConnector,
    bybit: BybitConnector,
    state: Arc<SharedState>,
    risk_engine: Arc<RiskEngine>,
    min_annualized_spread: f64,
    check_interval: Duration,
    dry_run: bool,
}

impl FundingArbStrategy {
    pub fn new(
        hyperliquid: HyperliquidConnector,
        binance: BinanceConnector,
        bybit: BybitConnector,
        state: Arc<SharedState>,
        risk_engine: Arc<RiskEngine>,
        min_annualized_spread: f64,
        check_interval_ms: u64,
        dry_run: bool,
    ) -> Self {
        Self {
            hyperliquid,
            binance,
            bybit,
            state,
            risk_engine,
            min_annualized_spread,
            check_interval: Duration::from_millis(check_interval_ms),
            dry_run,
        }
    }

    pub async fn run_loop(&self) {
        info!("Starting funding arbitrage loop");
        
        // Discover top symbols
        let symbols = match self.discover_symbols().await {
            Ok(syms) => syms,
            Err(e) => {
                error!("Failed to discover symbols: {}", e);
                return;
            }
        };

        info!("Monitoring {} symbols for funding arbitrage", symbols.len());

        loop {
            if self.risk_engine.is_kill_switch_active() {
                warn!("Kill switch active, pausing funding arb loop");
                sleep(Duration::from_secs(10)).await;
                continue;
            }

            for symbol in &symbols {
                if let Err(e) = self.check_opportunity(symbol).await {
                    error!("Error checking opportunity for {}: {}", symbol, e);
                }
            }

            sleep(self.check_interval).await;
        }
    }

    async fn discover_symbols(&self) -> Result<Vec<String>> {
        let hl_symbols = self.hyperliquid.get_top_symbols_by_volume(10).await?;
        let binance_symbols = self.binance.get_top_symbols_by_volume(10).await?;
        let bybit_symbols = self.bybit.get_top_symbols_by_volume(10).await?;

        // Find intersection of symbols across all venues
        let mut common: Vec<String> = hl_symbols
            .iter()
            .filter(|s| binance_symbols.contains(s) && bybit_symbols.contains(s))
            .cloned()
            .collect();

        if common.is_empty() {
            // Fallback: use Hyperliquid symbols
            common = hl_symbols;
        }

        Ok(common.into_iter().take(10).collect())
    }

    async fn check_opportunity(&self, symbol: &str) -> Result<()> {
        // Fetch funding rates from all venues
        let hl_funding = self.hyperliquid.get_funding_rate(symbol).await?;
        let binance_funding = self.binance.get_funding_rate(symbol).await?;
        let bybit_funding = self.bybit.get_funding_rate(symbol).await?;

        // Update state
        self.state.funding_rates.insert(
            (symbol.to_string(), Venue::Hyperliquid),
            hl_funding.clone(),
        );
        self.state.funding_rates.insert(
            (symbol.to_string(), Venue::Binance),
            binance_funding.clone(),
        );
        self.state.funding_rates.insert(
            (symbol.to_string(), Venue::Bybit),
            bybit_funding.clone(),
        );

        // Find the highest and lowest funding rates
        let venues = vec![
            ("Hyperliquid", hl_funding.rate, Venue::Hyperliquid),
            ("Binance", binance_funding.rate, Venue::Binance),
            ("Bybit", bybit_funding.rate, Venue::Bybit),
        ];

        let (high_venue_name, high_rate, high_venue) = venues
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        let (low_venue_name, low_rate, low_venue) = venues
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        // Calculate annualized spread
        let spread = high_rate - low_rate;
        let annualized_spread = spread * Decimal::from(365 * 3); // Funding is typically every 8 hours

        let annualized_f64 = annualized_spread.to_f64().unwrap_or(0.0);
        if annualized_f64 < self.min_annualized_spread {
            return Ok(());
        }

        info!(
            "Funding arb opportunity found: {} - {}: {:.4}% vs {}: {:.4}% (annualized: {:.2}%)",
            symbol,
            high_venue_name,
            high_rate.to_f64().unwrap_or(0.0) * 100.0,
            low_venue_name,
            low_rate.to_f64().unwrap_or(0.0) * 100.0,
            annualized_f64 * 100.0
        );

        if self.dry_run {
            info!("DRY RUN: Would execute funding arbitrage");
            return Ok(());
        }

        // Execute: short on high-funding venue, long on low-funding venue
        self.execute_funding_arb(symbol, high_venue, low_venue, spread).await?;

        Ok(())
    }

    async fn execute_funding_arb(
        &self,
        symbol: &str,
        high_venue: &Venue,
        low_venue: &Venue,
        spread: Decimal,
    ) -> Result<()> {
        // Determine position size (simplified - in production, use risk engine limits)
        let position_size = Decimal::from(1000u64); // Placeholder

        // Place orders
        let short_trade = match high_venue {
            Venue::Hyperliquid => {
                self.hyperliquid
                    .place_order(symbol, PositionSide::Short, position_size, Decimal::ZERO)
                    .await?
            }
            Venue::Binance => {
                // Binance connector would need order placement method
                warn!("Binance order placement not implemented");
                return Ok(());
            }
            Venue::Bybit => {
                // Bybit connector would need order placement method
                warn!("Bybit order placement not implemented");
                return Ok(());
            }
            _ => return Err(anyhow::anyhow!("Unsupported venue for short")),
        };

        let long_trade = match low_venue {
            Venue::Hyperliquid => {
                self.hyperliquid
                    .place_order(symbol, PositionSide::Long, position_size, Decimal::ZERO)
                    .await?
            }
            Venue::Binance => {
                warn!("Binance order placement not implemented");
                return Ok(());
            }
            Venue::Bybit => {
                warn!("Bybit order placement not implemented");
                return Ok(());
            }
            _ => return Err(anyhow::anyhow!("Unsupported venue for long")),
        };

        // Check risk before executing
        if let Err(e) = self.risk_engine.check_trade(&short_trade) {
            warn!("Risk check failed for short trade: {}", e);
            return Ok(());
        }

        if let Err(e) = self.risk_engine.check_trade(&long_trade) {
            warn!("Risk check failed for long trade: {}", e);
            return Ok(());
        }

        // Record trades
        self.risk_engine.record_trade(&short_trade);
        self.risk_engine.record_trade(&long_trade);

        info!(
            "Executed funding arbitrage: Short {} on {:?}, Long {} on {:?}",
            symbol, high_venue, symbol, low_venue
        );

        Ok(())
    }
}
