use crate::config::RiskConfig;
use crate::types::{Trade, Venue};
use parking_lot::RwLock;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RiskEngine {
    config: RiskConfig,
    daily_pnl: Arc<RwLock<Decimal>>,
    position_counts: Arc<RwLock<HashMap<Venue, usize>>>,
    notional_exposure: Arc<RwLock<HashMap<String, Decimal>>>,
    kill_switch_active: Arc<RwLock<bool>>,
}

impl RiskEngine {
    pub fn new(config: RiskConfig) -> Self {
        Self {
            config,
            daily_pnl: Arc::new(RwLock::new(Decimal::ZERO)),
            position_counts: Arc::new(RwLock::new(HashMap::new())),
            notional_exposure: Arc::new(RwLock::new(HashMap::new())),
            kill_switch_active: Arc::new(RwLock::new(false)),
        }
    }

    pub fn check_trade(&self, trade: &Trade) -> Result<(), RiskError> {
        // Check kill switch
        if *self.kill_switch_active.read() {
            return Err(RiskError::KillSwitchActive);
        }

        // Check daily PnL threshold
        let pnl = *self.daily_pnl.read();
        if pnl < Decimal::from_f64_retain(self.config.kill_switch_daily_loss_threshold).unwrap() {
            *self.kill_switch_active.write() = true;
            return Err(RiskError::DailyLossThresholdExceeded);
        }

        // Check position count per venue
        let counts = self.position_counts.write();
        let count = counts.get(&trade.venue).copied().unwrap_or(0);
        if count >= self.config.max_open_positions_per_venue {
            return Err(RiskError::MaxPositionsExceeded);
        }

        // Check notional exposure per asset
        let notional = trade.size * trade.price;
        let exposure = self.notional_exposure.write();
        let current_exposure = exposure.get(&trade.symbol).copied().unwrap_or(Decimal::ZERO);
        let max_notional = Decimal::from_f64_retain(self.config.max_notional_per_asset).unwrap();
        
        if current_exposure + notional > max_notional {
            return Err(RiskError::MaxNotionalExceeded);
        }

        Ok(())
    }

    pub fn record_trade(&self, trade: &Trade) {
        if trade.status == crate::types::TradeStatus::Filled {
            let mut counts = self.position_counts.write();
            *counts.entry(trade.venue.clone()).or_insert(0) += 1;

            let mut exposure = self.notional_exposure.write();
            let notional = trade.size * trade.price;
            *exposure.entry(trade.symbol.clone()).or_insert(Decimal::ZERO) += notional;
        }
    }

    pub fn record_position_closed(&self, venue: &Venue, symbol: &str, notional: Decimal) {
        let mut counts = self.position_counts.write();
        if let Some(count) = counts.get_mut(venue) {
            *count = count.saturating_sub(1);
        }

        let mut exposure = self.notional_exposure.write();
        if let Some(current) = exposure.get_mut(symbol) {
            *current = current.saturating_sub(notional);
        }
    }

    pub fn update_daily_pnl(&self, pnl_delta: Decimal) {
        let mut pnl = self.daily_pnl.write();
        *pnl += pnl_delta;
    }

    pub fn is_kill_switch_active(&self) -> bool {
        *self.kill_switch_active.read()
    }

    pub fn reset_daily_pnl(&self) {
        *self.daily_pnl.write() = Decimal::ZERO;
        *self.kill_switch_active.write() = false;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RiskError {
    #[error("Kill switch is active")]
    KillSwitchActive,
    #[error("Daily loss threshold exceeded")]
    DailyLossThresholdExceeded,
    #[error("Maximum positions exceeded for venue")]
    MaxPositionsExceeded,
    #[error("Maximum notional exposure exceeded for asset")]
    MaxNotionalExceeded,
}
