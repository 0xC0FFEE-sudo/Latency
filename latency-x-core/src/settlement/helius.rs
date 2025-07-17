use crate::config::{HeliusConfig, SolanaConfig};
use crate::models::{Order, OrderSide};
use crate::settlement::Settlement;
use anyhow::Result;
use async_trait::async_trait;
use helius_sdk::{Cluster, Helius};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, instruction::Instruction, signature::Keypair,
    signer::Signer, transaction::Transaction as SolanaTransaction,
};
use std::sync::Arc;
use tracing::info;

use super::pump::{get_buy_instruction, get_sell_instruction};

pub struct HeliusSettlement {
    helius: Arc<Helius>,
    wallet: Keypair,
}

impl HeliusSettlement {
    pub fn new(config: &HeliusConfig, solana_config: &SolanaConfig) -> Result<Self> {
        let cluster = match config.cluster.as_str() {
            "mainnet-beta" => Cluster::MainnetBeta,
            "devnet" => Cluster::Devnet,
            _ => return Err(anyhow::anyhow!("Invalid Helius cluster")),
        };
        let helius = Arc::new(Helius::new(config.api_key.clone(), cluster));
        let wallet = Keypair::from_base58_string(&solana_config.private_key);
        Ok(Self { helius, wallet })
    }
}

#[async_trait]
impl Settlement for HeliusSettlement {
    async fn send_order(
        &self,
        order: &Order,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let blockhash = self
            .helius
            .rpc
            .connection()
            .get_latest_blockhash()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        let mint_address = &order.symbol;
        let token_amount = (order.amount * 1_000_000.0) as u64;
        let sol_amount = (order.price.unwrap_or(0.0) * order.amount * 1_000_000_000.0) as u64;

        let instruction = match order.side {
            OrderSide::Buy => get_buy_instruction(
                &self.wallet.pubkey(),
                mint_address,
                token_amount,
                sol_amount,
            )?,
            OrderSide::Sell => get_sell_instruction(
                &self.wallet.pubkey(),
                mint_address,
                token_amount,
                sol_amount,
            )?,
        };

        let instructions = vec![
            ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
            ComputeBudgetInstruction::set_compute_unit_price(5000),
            instruction,
        ];

        let mut tx = SolanaTransaction::new_with_payer(&instructions, Some(&self.wallet.pubkey()));
        tx.sign(&[&self.wallet], blockhash);

        let signature = self
            .helius
            .rpc
            .connection()
            .send_and_confirm_transaction(&tx)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        info!("Transaction sent with signature: {:?}", signature);

        Ok(signature.to_string())
    }
} 