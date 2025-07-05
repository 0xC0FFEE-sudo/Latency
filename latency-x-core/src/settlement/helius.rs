use crate::models::Order;
use crate::settlement::SettlementHandler;
use anyhow::Result;
use async_trait::async_trait;
use helius_sdk::{Cluster, Helius};
use crate::config::HeliusConfig;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use tracing::info;

pub struct HeliusSettlementHandler {
    client: Helius,
    signer: Keypair,
}

impl HeliusSettlementHandler {
    pub fn new(config: &HeliusConfig, solana_config: &crate::config::SolanaConfig) -> Result<Self> {
        let client = Helius::new(config.api_key.clone(), Cluster::MainnetBeta);
        let signer = Keypair::from_base58_string(&solana_config.private_key);
        Ok(Self { client, signer })
    }
}

#[async_trait]
impl SettlementHandler for HeliusSettlementHandler {
    async fn settle(&self, order: &Order) -> Result<String> {
        info!("[SETTLEMENT] Attempting to settle order {:?} on Helius.", order.id);
        
        // This is a placeholder for creating a real transaction.
        // The actual transaction creation would depend on the specific program we are interacting with.
        // For now, we'll simulate a simple transfer to demonstrate the Helius integration.
        let to_pubkey = solana_sdk::pubkey::Pubkey::from_str(&order.destination_address)?;
        let instruction = solana_sdk::system_instruction::transfer(
            &self.signer.pubkey(),
            &to_pubkey,
            1_000_000, // 0.001 SOL
        );

        let latest_blockhash = self.client.rpc.connection().get_latest_blockhash()?;
        
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.signer.pubkey()),
            &[&self.signer],
            latest_blockhash,
        );

        let signature = self.client.rpc.connection().send_transaction(&tx)?;

        info!("[SETTLEMENT] Successfully sent transaction via Helius: {}", signature);
        info!("[SETTLEMENT] Order {:?} processed by Helius handler.", order.id);

        Ok(signature.to_string())
    }
} 