use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use std::str::FromStr;
use std::collections::HashMap;
use base64::prelude::*;
const JUPITER_QUOTE_API: &str = "https://quote-api.jup.ag/v6";

#[derive(Debug, Deserialize)]
struct IndexedRouteMap {
    #[serde(rename = "mintKeys")]
    mint_keys: Vec<String>,
    #[serde(rename = "indexedRouteMap")]
    indexed_route_map: HashMap<String, Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    #[serde(rename = "inputMint")]
    input_mint: String,
    #[serde(rename = "outputMint")]
    output_mint: String,
    #[serde(rename = "inAmount")]
    in_amount: String,
    #[serde(rename = "outAmount")]
    out_amount: String,
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SwapRequest {
    #[serde(rename = "quoteResponse")]
    quote_response: Quote,
    #[serde(rename = "userPublicKey")]
    user_public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SwapResponse {
    #[serde(rename = "swapTransaction")]
    swap_transaction: String,
}

pub struct JupiterTrader {
    client: RpcClient,
    wallet: Keypair,
    route_map: Option<IndexedRouteMap>,
}

impl JupiterTrader {
    pub fn new(rpc_url: &str, private_key: &str) -> Result<Self> {
        let client = RpcClient::new(rpc_url.to_string());
        let wallet = Keypair::from_base58_string(private_key);
        
        Ok(Self {
            client,
            wallet,
            route_map: None,
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        // Fetch and cache the route map
        let route_map: IndexedRouteMap = reqwest::get(format!("{}/indexed-route-map", JUPITER_QUOTE_API))
            .await?
            .json()
            .await?;
        self.route_map = Some(route_map);
        Ok(())
    }

    pub async fn get_price(&self, input_mint: &str, output_mint: &str, amount: u64) -> Result<f64> {
        let quote: Quote = reqwest::get(format!(
            "{}/quote?inputMint={}&outputMint={}&amount={}&slippageBps=10",
            JUPITER_QUOTE_API, input_mint, output_mint, amount
        ))
        .await?
        .json()
        .await?;

        Ok(quote.price)
    }

    pub async fn swap(&self, input_mint: &str, output_mint: &str, amount: u64) -> Result<String> {
        // Get quote
        let quote: Quote = reqwest::get(format!(
            "{}/quote?inputMint={}&outputMint={}&amount={}&slippageBps=10",
            JUPITER_QUOTE_API, input_mint, output_mint, amount
        ))
        .await?
        .json()
        .await?;

        // Get swap transaction
        let swap_request = SwapRequest {
            quote_response: quote,
            user_public_key: self.wallet.pubkey().to_string(),
        };

        let swap_response: SwapResponse = reqwest::Client::new()
            .post(format!("{}/swap", JUPITER_QUOTE_API))
            .json(&swap_request)
            .send()
            .await?
            .json()
            .await?;

        // Deserialize and sign transaction
        let tx_buf = BASE64_STANDARD.decode(&swap_response.swap_transaction)?;
        let versioned_transaction: VersionedTransaction = bincode::deserialize(&tx_buf)?;
        let signed_version_transaction = VersionedTransaction::try_new(versioned_transaction.message, &[&self.wallet])?;
        let signature = self.client.send_and_confirm_transaction(&signed_version_transaction)?;
        Ok(signature.to_string())
    }

    pub fn get_token_balance(&self, token_mint: &str) -> Result<f64> {
        let token_pubkey = Pubkey::from_str(token_mint)?;
        
        let accounts = self.client.get_token_accounts_by_owner(
            &self.wallet.pubkey(), 
            solana_client::rpc_request::TokenAccountsFilter::Mint(token_pubkey),
        )?;

        for account in accounts {
            let balance = self.client.get_token_account_balance(&Pubkey::from_str(&account.pubkey)?)?;
            return Ok(balance.ui_amount.unwrap_or(0.0));
        }

        Ok(0.0)
    }

    pub fn get_available_routes(&self, token_mint: &str) -> Result<Vec<String>> {
        let route_map = self.route_map.as_ref()
            .ok_or_else(|| anyhow!("Route map not initialized. Call init() first"))?;

        let token_index = route_map.mint_keys
            .iter()
            .position(|mint| mint == token_mint)
            .ok_or_else(|| anyhow!("Token mint not found in route map"))?;

        let routes = route_map.indexed_route_map
            .get(&token_index.to_string())
            .ok_or_else(|| anyhow!("No routes found for token"))?;

        Ok(routes
            .iter()
            .map(|&index| route_map.mint_keys[index as usize].clone())
            .collect())
    }
}
