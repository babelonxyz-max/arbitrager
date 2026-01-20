use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub risk: RiskConfig,
    pub hyperliquid: HyperliquidConfig,
    pub binance: BinanceConfig,
    pub bybit: BybitConfig,
    pub hyperevm: HyperEvmConfig,
    pub solana: SolanaConfig,
    pub strategies: StrategiesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub dry_run: bool,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_log_level() -> String {
    "info".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    pub max_notional_per_asset: f64,
    pub max_open_positions_per_venue: usize,
    pub max_leverage: f64,
    pub kill_switch_daily_loss_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperliquidConfig {
    pub api_url: String,
    #[serde(default)]
    pub private_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceConfig {
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub api_secret: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BybitConfig {
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub api_secret: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperEvmConfig {
    pub rpc_url: String,
    #[serde(default)]
    pub dex_router_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    #[serde(default)]
    pub private_key: String,
    pub jupiter_api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategiesConfig {
    pub funding_arb_enabled: bool,
    pub hyperevm_spot_enabled: bool,
    pub solana_jupiter_enabled: bool,
    #[serde(default)]
    pub funding_arb: FundingArbConfig,
    #[serde(default)]
    pub hyperevm_spot: HyperEvmSpotConfig,
    #[serde(default)]
    pub solana_jupiter: SolanaJupiterConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingArbConfig {
    #[serde(default = "default_min_annualized_spread")]
    pub min_annualized_spread: f64,
    #[serde(default = "default_check_interval_ms")]
    pub check_interval_ms: u64,
}

impl Default for FundingArbConfig {
    fn default() -> Self {
        Self {
            min_annualized_spread: default_min_annualized_spread(),
            check_interval_ms: default_check_interval_ms(),
        }
    }
}

fn default_min_annualized_spread() -> f64 {
    0.05
}

fn default_check_interval_ms() -> u64 {
    10000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperEvmSpotConfig {
    #[serde(default = "default_min_spread_bps")]
    pub min_spread_bps: u64,
    #[serde(default = "default_check_interval_ms")]
    pub check_interval_ms: u64,
}

impl Default for HyperEvmSpotConfig {
    fn default() -> Self {
        Self {
            min_spread_bps: default_min_spread_bps(),
            check_interval_ms: default_check_interval_ms(),
        }
    }
}

fn default_min_spread_bps() -> u64 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaJupiterConfig {
    #[serde(default = "default_min_profit_bps")]
    pub min_profit_bps: u64,
    #[serde(default = "default_check_interval_ms")]
    pub check_interval_ms: u64,
    #[serde(default = "default_max_slippage_bps")]
    pub max_slippage_bps: u64,
}

impl Default for SolanaJupiterConfig {
    fn default() -> Self {
        Self {
            min_profit_bps: default_min_profit_bps(),
            check_interval_ms: default_check_interval_ms(),
            max_slippage_bps: default_max_slippage_bps(),
        }
    }
}

fn default_min_profit_bps() -> u64 {
    20
}

fn default_max_slippage_bps() -> u64 {
    50
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config from {:?}", path.as_ref()))?;
        let mut config: Config = toml::from_str(&content)
            .context("Failed to parse config TOML")?;

        // Override with environment variables if present
        if let Ok(key) = std::env::var("HYPERLIQUID_PRIVATE_KEY") {
            config.hyperliquid.private_key = key;
        }
        if let Ok(key) = std::env::var("BINANCE_API_KEY") {
            config.binance.api_key = key;
        }
        if let Ok(secret) = std::env::var("BINANCE_API_SECRET") {
            config.binance.api_secret = secret;
        }
        if let Ok(key) = std::env::var("BYBIT_API_KEY") {
            config.bybit.api_key = key;
        }
        if let Ok(secret) = std::env::var("BYBIT_API_SECRET") {
            config.bybit.api_secret = secret;
        }
        if let Ok(key) = std::env::var("SOLANA_PRIVATE_KEY") {
            config.solana.private_key = key;
        }

        Ok(config)
    }
}
