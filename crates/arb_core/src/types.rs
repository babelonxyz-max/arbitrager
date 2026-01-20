use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub venue: Venue,
    pub price: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Venue {
    Hyperliquid,
    Binance,
    Bybit,
    HyperEvm,
    SolanaJupiter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRate {
    pub symbol: String,
    pub venue: Venue,
    pub rate: Decimal,
    pub predicted_rate: Option<Decimal>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub strategy: StrategyType,
    pub symbol: String,
    pub venue_a: Venue,
    pub venue_b: Venue,
    pub price_a: Decimal,
    pub price_b: Decimal,
    pub spread_bps: i64,
    pub estimated_profit: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StrategyType {
    FundingArb,
    HyperEvmSpot,
    SolanaJupiter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub venue: Venue,
    pub side: PositionSide,
    pub size: Decimal,
    pub entry_price: Decimal,
    pub leverage: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PositionSide {
    Long,
    Short,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub venue: Venue,
    pub side: PositionSide,
    pub size: Decimal,
    pub price: Decimal,
    pub timestamp: DateTime<Utc>,
    pub status: TradeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeStatus {
    Pending,
    Filled,
    Rejected,
    Failed,
}

#[derive(Debug, Clone)]
pub struct SharedState {
    pub market_data: dashmap::DashMap<(String, Venue), MarketData>,
    pub funding_rates: dashmap::DashMap<(String, Venue), FundingRate>,
    pub positions: dashmap::DashMap<String, Position>,
    pub trades: dashmap::DashMap<String, Trade>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            market_data: dashmap::DashMap::new(),
            funding_rates: dashmap::DashMap::new(),
            positions: dashmap::DashMap::new(),
            trades: dashmap::DashMap::new(),
        }
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}
