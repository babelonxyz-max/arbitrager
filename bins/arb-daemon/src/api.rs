use arb_core::{RiskEngine, SharedState};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use chrono::Utc;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct ApiState {
    pub shared_state: Arc<SharedState>,
    pub risk_engine: Arc<RiskEngine>,
    pub strategies_enabled: StrategiesConfig,
    pub dry_run: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StrategiesConfig {
    pub funding_arb: bool,
    pub hyperevm_spot: bool,
    pub solana_jupiter: bool,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub strategies: StrategiesConfig,
    pub kill_switch_active: bool,
    pub dry_run: bool,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct OpportunitiesResponse {
    pub opportunities: Vec<OpportunityResponse>,
}

#[derive(Serialize)]
pub struct OpportunityResponse {
    pub strategy: String,
    pub symbol: String,
    pub venue_a: String,
    pub venue_b: String,
    pub spread_bps: i64,
    pub estimated_profit: f64,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponse>,
}

#[derive(Serialize)]
pub struct PositionResponse {
    pub symbol: String,
    pub venue: String,
    pub side: String,
    pub size: f64,
    pub entry_price: f64,
    pub leverage: f64,
}

pub fn create_router(state: ApiState) -> Router {
    Router::new()
        .route("/api/status", get(get_status))
        .route("/api/opportunities", get(get_opportunities))
        .route("/api/positions", get(get_positions))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn get_status(State(state): State<ApiState>) -> Result<Json<StatusResponse>, StatusCode> {
    let status = if state.risk_engine.is_kill_switch_active() {
        "error"
    } else {
        "running"
    };

    Ok(Json(StatusResponse {
        status: status.to_string(),
        strategies: state.strategies_enabled.clone(),
        kill_switch_active: state.risk_engine.is_kill_switch_active(),
        dry_run: state.dry_run,
        timestamp: Utc::now().to_rfc3339(),
    }))
}

async fn get_opportunities(
    State(state): State<ApiState>,
) -> Result<Json<OpportunitiesResponse>, StatusCode> {
    // In a real implementation, we'd track opportunities in SharedState
    // For now, return empty array - opportunities would be logged but not stored
    Ok(Json(OpportunitiesResponse {
        opportunities: vec![],
    }))
}

async fn get_positions(
    State(state): State<ApiState>,
) -> Result<Json<PositionsResponse>, StatusCode> {
    let positions: Vec<PositionResponse> = state
        .shared_state
        .positions
        .iter()
        .map(|entry| {
            let pos = entry.value();
            PositionResponse {
                symbol: pos.symbol.clone(),
                venue: format!("{:?}", pos.venue),
                side: format!("{:?}", pos.side),
                size: pos.size.to_f64().unwrap_or(0.0),
                entry_price: pos.entry_price.to_f64().unwrap_or(0.0),
                leverage: pos.leverage.to_f64().unwrap_or(0.0),
            }
        })
        .collect();

    Ok(Json(PositionsResponse { positions }))
}
