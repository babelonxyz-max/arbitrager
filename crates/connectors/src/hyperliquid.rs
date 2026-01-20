use arb_core::types::{FundingRate, MarketData, Position, PositionSide, Trade, TradeStatus, Venue};
use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct HyperliquidConnector {
    client: Client,
    api_url: String,
    private_key: String,
}

impl HyperliquidConnector {
    pub fn new(api_url: String, private_key: String) -> Self {
        Self {
            client: Client::new(),
            api_url,
            private_key,
        }
    }

    pub async fn get_market_data(&self, symbol: &str) -> Result<MarketData> {
        let url = format!("{}/info", self.api_url);
        let payload = serde_json::json!({
            "type": "l2Book",
            "coin": symbol
        });

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .context("Failed to fetch Hyperliquid market data")?;

        let data: HyperliquidL2Book = resp.json().await?;
        
        let mid_price = if !data.levels.is_empty() {
            let best_bid = data.levels[0].px.parse::<f64>()?;
            let best_ask = data.levels[0].sz.parse::<f64>()?;
            Decimal::try_from((best_bid + best_ask) / 2.0)
                .unwrap_or(Decimal::ZERO)
        } else {
            Decimal::ZERO
        };

        Ok(MarketData {
            symbol: symbol.to_string(),
            venue: Venue::Hyperliquid,
            price: mid_price,
            timestamp: Utc::now(),
        })
    }

    pub async fn get_funding_rate(&self, symbol: &str) -> Result<FundingRate> {
        let url = format!("{}/info", self.api_url);
        let payload = serde_json::json!({
            "type": "metaAndAssetCtxs"
        });

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .context("Failed to fetch Hyperliquid funding rate")?;

        let data: HyperliquidMeta = resp.json().await?;
        
        // Find the asset in the response
        let asset_ctx = data
            .asset_ctxs
            .iter()
            .find(|a| a.name == symbol)
            .context(format!("Symbol {} not found", symbol))?;

        let rate = Decimal::from_str_exact(&asset_ctx.funding)
            .or_else(|_| Decimal::from_str(&asset_ctx.funding))
            .context("Failed to parse funding rate")?;
        let predicted = asset_ctx.predicted_funding.and_then(|p| {
            Decimal::from_str_exact(&p).or_else(|_| Decimal::from_str(&p)).ok()
        });

        Ok(FundingRate {
            symbol: symbol.to_string(),
            venue: Venue::Hyperliquid,
            rate,
            predicted_rate: predicted,
            timestamp: Utc::now(),
        })
    }

    pub async fn get_top_symbols_by_volume(&self, limit: usize) -> Result<Vec<String>> {
        let url = format!("{}/info", self.api_url);
        let payload = serde_json::json!({
            "type": "metaAndAssetCtxs"
        });

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .context("Failed to fetch Hyperliquid symbols")?;

        let data: HyperliquidMeta = resp.json().await?;
        
        let mut symbols: Vec<(String, f64)> = data
            .asset_ctxs
            .iter()
            .map(|a| (a.name.clone(), a.volume_24h.unwrap_or(0.0)))
            .collect();
        
        symbols.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(symbols.into_iter().take(limit).map(|(s, _)| s).collect())
    }

    pub async fn place_order(
        &self,
        symbol: &str,
        side: PositionSide,
        size: Decimal,
        price: Decimal,
    ) -> Result<Trade> {
        // In a real implementation, this would sign and submit orders via Hyperliquid API
        // For now, we'll simulate the order placement
        warn!("Hyperliquid order placement not fully implemented - simulating");
        
        Ok(Trade {
            symbol: symbol.to_string(),
            venue: Venue::Hyperliquid,
            side,
            size,
            price,
            timestamp: Utc::now(),
            status: TradeStatus::Pending,
        })
    }
}

#[derive(Debug, Deserialize)]
struct HyperliquidL2Book {
    levels: Vec<Level>,
}

#[derive(Debug, Deserialize)]
struct Level {
    px: String,
    sz: String,
}

#[derive(Debug, Deserialize)]
struct HyperliquidMeta {
    asset_ctxs: Vec<AssetCtx>,
}

#[derive(Debug, Deserialize)]
struct AssetCtx {
    name: String,
    funding: String,
    #[serde(rename = "predictedFunding")]
    predicted_funding: Option<String>,
    #[serde(rename = "volume24h")]
    volume_24h: Option<f64>,
}
