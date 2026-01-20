mod api;

use arb_core::{Config, RiskEngine, SharedState};
use api::{ApiState, StrategiesConfig};
use connectors::{BinanceConnector, BybitConnector, HyperEvmConnector, HyperliquidConnector, JupiterConnector};
use std::sync::Arc;
use strategies::{FundingArbStrategy, HyperEvmSpotStrategy, SolanaJupiterStrategy};
use tokio::signal;
use tracing::{error, info};
use axum::serve;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Starting arbitrage daemon");

    // Load configuration
    let config_path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "config/local.toml".to_string());
    
    let config = Config::load(&config_path)?;
    
    if config.general.dry_run {
        info!("DRY RUN MODE: No trades will be executed");
    }

    // Initialize shared state
    let state = Arc::new(SharedState::new());
    let risk_engine = Arc::new(RiskEngine::new(config.risk.clone()));

    // Initialize connectors
    let hyperliquid = HyperliquidConnector::new(
        config.hyperliquid.api_url.clone(),
        config.hyperliquid.private_key.clone(),
    );

    let binance = BinanceConnector::new(
        config.binance.api_key.clone(),
        config.binance.api_secret.clone(),
        config.binance.base_url.clone(),
    );

    let bybit = BybitConnector::new(
        config.bybit.api_key.clone(),
        config.bybit.api_secret.clone(),
        config.bybit.base_url.clone(),
    );

    let hyperevm = match HyperEvmConnector::new(
        config.hyperevm.rpc_url.clone(),
        if config.hyperevm.dex_router_address.is_empty() {
            None
        } else {
            Some(config.hyperevm.dex_router_address.clone())
        },
    ) {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to initialize HyperEVM connector: {}", e);
            return Err(e);
        }
    };

    let jupiter = match JupiterConnector::new(
        config.solana.rpc_url.clone(),
        config.solana.jupiter_api_url.clone(),
        if config.solana.private_key.is_empty() {
            None
        } else {
            Some(config.solana.private_key.clone())
        },
    ) {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to initialize Jupiter connector: {}", e);
            return Err(e);
        }
    };

    // Initialize strategies
    let mut strategy_tasks = Vec::new();

    if config.strategies.funding_arb_enabled {
        info!("Enabling funding arbitrage strategy");
        let funding_strategy = FundingArbStrategy::new(
            hyperliquid.clone(),
            binance.clone(),
            bybit.clone(),
            state.clone(),
            risk_engine.clone(),
            config.strategies.funding_arb.min_annualized_spread,
            config.strategies.funding_arb.check_interval_ms,
            config.general.dry_run,
        );
        strategy_tasks.push(tokio::spawn(async move {
            funding_strategy.run_loop().await;
        }));
    }

    if config.strategies.hyperevm_spot_enabled {
        info!("Enabling HyperEVM spot arbitrage strategy");
        let hyperevm_strategy = HyperEvmSpotStrategy::new(
            hyperliquid.clone(),
            hyperevm.clone(),
            state.clone(),
            risk_engine.clone(),
            config.strategies.hyperevm_spot.min_spread_bps,
            config.strategies.hyperevm_spot.check_interval_ms,
            config.general.dry_run,
        );
        strategy_tasks.push(tokio::spawn(async move {
            hyperevm_strategy.run_loop().await;
        }));
    }

    if config.strategies.solana_jupiter_enabled {
        info!("Enabling Solana Jupiter arbitrage strategy");
        let solana_strategy = SolanaJupiterStrategy::new(
            jupiter.clone(),
            state.clone(),
            risk_engine.clone(),
            config.strategies.solana_jupiter.min_profit_bps,
            config.strategies.solana_jupiter.max_slippage_bps,
            config.strategies.solana_jupiter.check_interval_ms,
            config.general.dry_run,
        );
        strategy_tasks.push(tokio::spawn(async move {
            solana_strategy.run_loop().await;
        }));
    }

    if strategy_tasks.is_empty() {
        error!("No strategies enabled. Please enable at least one strategy in config.");
        return Err(anyhow::anyhow!("No strategies enabled"));
    }

    // Start API server
    let api_state = ApiState {
        shared_state: state.clone(),
        risk_engine: risk_engine.clone(),
        strategies_enabled: StrategiesConfig {
            funding_arb: config.strategies.funding_arb_enabled,
            hyperevm_spot: config.strategies.hyperevm_spot_enabled,
            solana_jupiter: config.strategies.solana_jupiter_enabled,
        },
        dry_run: config.general.dry_run,
    };

    let api_router = api::create_router(api_state);
    let api_addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    
    let api_server = tokio::spawn(async move {
        info!("Starting API server on http://0.0.0.0:8080");
        let listener = match TcpListener::bind(&api_addr).await {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to bind to {}: {}", api_addr, e);
                return;
            }
        };
        
        if let Err(e) = serve(listener, api_router).await {
            error!("API server error: {}", e);
        }
    });

    info!("All strategies started. API server running on port 8080. Waiting for shutdown signal...");

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutdown signal received");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Cancel all tasks
    api_server.abort();
    for task in strategy_tasks {
        task.abort();
    }

    info!("Arbitrage daemon stopped");
    Ok(())
}
