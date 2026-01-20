use arb_core::types::{MarketData, Venue};
use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Keypair,
    transaction::Transaction,
};
use std::str::FromStr;
use std::sync::Arc;
use tracing::warn;

pub struct JupiterConnector {
    client: Client,
    rpc_client: Arc<RpcClient>,
    jupiter_api_url: String,
    _keypair: Option<Keypair>, // Not used yet, but kept for future implementation
}

impl Clone for JupiterConnector {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            rpc_client: Arc::clone(&self.rpc_client),
            jupiter_api_url: self.jupiter_api_url.clone(),
            _keypair: None, // Keypair doesn't implement Clone, set to None on clone
        }
    }
}

impl JupiterConnector {
    pub fn new(rpc_url: String, jupiter_api_url: String, private_key: Option<String>) -> Result<Self> {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            rpc_url.clone(),
            CommitmentConfig::confirmed(),
        ));

        // In production, properly parse the private key from string
        // For now, we'll leave it as None since Keypair doesn't implement Clone
        let _keypair: Option<Keypair> = None; // TODO: Parse private key if provided

        Ok(Self {
            client: Client::new(),
            rpc_client,
            jupiter_api_url,
            _keypair,
        })
    }

    pub async fn get_quote(
        &self,
        input_mint: &str,
        output_mint: &str,
        amount: u64,
        slippage_bps: u64,
    ) -> Result<JupiterQuote> {
        let url = format!("{}/quote", self.jupiter_api_url);
        let params = [
            ("inputMint", input_mint),
            ("outputMint", output_mint),
            ("amount", &amount.to_string()),
            ("slippageBps", &slippage_bps.to_string()),
        ];

        let resp = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .context("Failed to fetch Jupiter quote")?;

        let quote: JupiterQuote = resp.json().await?;
        Ok(quote)
    }

    pub async fn get_swap_instructions(
        &self,
        quote: &JupiterQuote,
        user_public_key: &str,
        slippage_bps: u64,
    ) -> Result<JupiterSwapResponse> {
        let url = format!("{}/swap", self.jupiter_api_url);
        let payload = serde_json::json!({
            "quoteResponse": quote,
            "userPublicKey": user_public_key,
            "slippageBps": slippage_bps,
        });

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .context("Failed to get Jupiter swap instructions")?;

        let swap: JupiterSwapResponse = resp.json().await?;
        Ok(swap)
    }

    pub async fn execute_swap(&self, swap_response: &JupiterSwapResponse) -> Result<String> {
        // In a real implementation, this would:
        // 1. Deserialize the transaction from swap_response
        // 2. Sign it with the keypair
        // 3. Send it to the Solana RPC
        // 4. Return the transaction signature
        
        warn!("Jupiter swap execution not fully implemented - simulating");
        Ok("simulated_tx_signature".to_string())
    }

    pub fn get_spot_price_from_quote(&self, quote: &JupiterQuote) -> MarketData {
        // Extract price from quote (simplified)
        let price = if quote.in_amount > 0 {
            Decimal::from(quote.out_amount) / Decimal::from(quote.in_amount)
        } else {
            Decimal::ZERO
        };
        
        MarketData {
            symbol: format!("{}/{}", quote.input_mint, quote.output_mint),
            venue: Venue::SolanaJupiter,
            price,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterQuote {
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    #[serde(rename = "inAmount")]
    pub in_amount: u64,
    #[serde(rename = "outAmount")]
    pub out_amount: u64,
    #[serde(rename = "otherAmountThreshold")]
    pub other_amount_threshold: u64,
    #[serde(rename = "swapMode")]
    pub swap_mode: String,
    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterSwapResponse {
    #[serde(rename = "swapTransaction")]
    pub swap_transaction: String,
}
