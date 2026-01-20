use arb_core::types::{FundingRate, Venue};
use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use chrono::Utc;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::Deserialize;
use tracing::warn;

#[derive(Clone)]
pub struct BybitConnector {
    client: Client,
    api_key: String,
    api_secret: String,
    base_url: String,
}

impl BybitConnector {
    pub fn new(api_key: String, api_secret: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_secret,
            base_url,
        }
    }

    pub async fn get_funding_rate(&self, symbol: &str) -> Result<FundingRate> {
        // Bybit uses BTCUSDT format
        let bybit_symbol = if symbol.contains("-") {
            symbol.replace("-", "")
        } else {
            symbol.to_string()
        };

        let url = format!("{}/v5/market/funding/history", self.base_url);
        let resp = self
            .client
            .get(&url)
            .query(&[("category", "linear"), ("symbol", &bybit_symbol), ("limit", "1")])
            .send()
            .await
            .context("Failed to fetch Bybit funding rate")?;

        let data: BybitFundingResponse = resp.json().await?;
        
        if data.result.list.is_empty() {
            anyhow::bail!("No funding rate data for {}", symbol);
        }

        let rate = Decimal::from_str_exact(&data.result.list[0].fundingRate)
            .or_else(|_| Decimal::from_str(&data.result.list[0].fundingRate))
            .context("Failed to parse funding rate")?;

        Ok(FundingRate {
            symbol: symbol.to_string(),
            venue: Venue::Bybit,
            rate,
            predicted_rate: None,
            timestamp: Utc::now(),
        })
    }

    pub async fn get_top_symbols_by_volume(&self, limit: usize) -> Result<Vec<String>> {
        let url = format!("{}/v5/market/tickers", self.base_url);
        let resp = self
            .client
            .get(&url)
            .query(&[("category", "linear")])
            .send()
            .await
            .context("Failed to fetch Bybit tickers")?;

        let data: BybitTickersResponse = resp.json().await?;
        
        let mut tickers = data.result.list;
        tickers.sort_by(|a, b| {
            b.turnover24h
                .parse::<f64>()
                .unwrap_or(0.0)
                .partial_cmp(&a.turnover24h.parse::<f64>().unwrap_or(0.0))
                .unwrap()
        });

        Ok(tickers
            .into_iter()
            .take(limit)
            .map(|t| t.symbol.replace("USDT", "-USDT"))
            .collect())
    }
}

#[derive(Debug, Deserialize)]
struct BybitFundingResponse {
    result: BybitFundingResult,
}

#[derive(Debug, Deserialize)]
struct BybitFundingResult {
    list: Vec<BybitFundingItem>,
}

#[derive(Debug, Deserialize)]
struct BybitFundingItem {
    #[serde(rename = "fundingRate")]
    fundingRate: String,
}

#[derive(Debug, Deserialize)]
struct BybitTickersResponse {
    result: BybitTickersResult,
}

#[derive(Debug, Deserialize)]
struct BybitTickersResult {
    list: Vec<BybitTicker>,
}

#[derive(Debug, Deserialize)]
struct BybitTicker {
    symbol: String,
    #[serde(rename = "turnover24h")]
    turnover24h: String,
}
