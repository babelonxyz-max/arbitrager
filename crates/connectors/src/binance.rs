use arb_core::types::{FundingRate, Venue};
use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use chrono::Utc;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::Deserialize;
use tracing::warn;

#[derive(Clone)]
pub struct BinanceConnector {
    client: Client,
    api_key: String,
    api_secret: String,
    base_url: String,
}

impl BinanceConnector {
    pub fn new(api_key: String, api_secret: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_secret,
            base_url,
        }
    }

    pub async fn get_funding_rate(&self, symbol: &str) -> Result<FundingRate> {
        // Binance uses BTCUSDT format, convert if needed
        let binance_symbol = if symbol.contains("-") {
            symbol.replace("-", "")
        } else {
            symbol.to_string()
        };

        let url = format!("{}/fapi/v1/premiumIndex", self.base_url);
        let resp = self
            .client
            .get(&url)
            .query(&[("symbol", &binance_symbol)])
            .send()
            .await
            .context("Failed to fetch Binance funding rate")?;

        let data: BinancePremiumIndex = resp.json().await?;
        
        let rate = Decimal::from_str_exact(&data.lastFundingRate)
            .or_else(|_| Decimal::from_str(&data.lastFundingRate))
            .context("Failed to parse funding rate")?;

        Ok(FundingRate {
            symbol: symbol.to_string(),
            venue: Venue::Binance,
            rate,
            predicted_rate: None,
            timestamp: Utc::now(),
        })
    }

    pub async fn get_top_symbols_by_volume(&self, limit: usize) -> Result<Vec<String>> {
        let url = format!("{}/fapi/v1/ticker/24hr", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch Binance tickers")?;

        let mut tickers: Vec<BinanceTicker> = resp.json().await?;
        
        // Filter for USDT pairs and sort by volume
        tickers.retain(|t| t.symbol.ends_with("USDT"));
        tickers.sort_by(|a, b| {
            b.quoteVolume
                .parse::<f64>()
                .unwrap_or(0.0)
                .partial_cmp(&a.quoteVolume.parse::<f64>().unwrap_or(0.0))
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
struct BinancePremiumIndex {
    #[serde(rename = "lastFundingRate")]
    lastFundingRate: String,
}

#[derive(Debug, Deserialize)]
struct BinanceTicker {
    symbol: String,
    #[serde(rename = "quoteVolume")]
    quoteVolume: String,
}
