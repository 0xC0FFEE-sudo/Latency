use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
    instruction::{Instruction, AccountMeta},
    system_program,
    sysvar::rent,

};
use std::{error::Error, str::FromStr, sync::Arc};
use crate::models::{Order, OrderSide};
use crate::execution::ExecutionGateway;
use async_trait::async_trait;
use crate::config::SolanaConfig;
use spl_associated_token_account::get_associated_token_address;
use borsh::{BorshSerialize, BorshDeserialize};

const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const GLOBAL_ACCOUNT_SEED: &[u8] = b"global";
const FEE_RECIPIENT_SEED: &[u8] = b"fee_recipient";
const BONDING_CURVE_SEED: &[u8] = b"bonding-curve";

pub struct PumpConnector {
    _rpc_client: Arc<RpcClient>,
    _signer: Keypair,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct BuyInstructionData {
    // Discriminator for the 'buy' instruction, not part of the struct
    // instruction: u8, 
    amount: u64,
    max_sol_cost: u64,
}

impl PumpConnector {
    #[allow(dead_code)]
    pub fn new(rpc_client: Arc<RpcClient>, config: &SolanaConfig) -> Self {
        let signer = Keypair::from_base58_string(&config.private_key);
        Self {
            _rpc_client: rpc_client,
            _signer: signer,
        }
    }

    fn create_buy_instruction(
        &self,
        mint_str: &str,
        token_amount: u64,
        max_sol_cost: u64,
    ) -> Result<Instruction, Box<dyn Error + Send + Sync>> {
        let program_id = Pubkey::from_str(PUMP_PROGRAM_ID)?;
        let token_program_id = Pubkey::from_str(TOKEN_PROGRAM_ID)?;
        let mint = Pubkey::from_str(mint_str)?;
        let sol_mint = spl_token::native_mint::id();
        
        let (global, _) = Pubkey::find_program_address(&[GLOBAL_ACCOUNT_SEED], &program_id);
        
        // The fee recipient is the associated token account for SOL for the fee_recipient address stored in the global account.
        // This requires fetching and parsing the global account data. For now, we'll find the fee_recipient PDA.
        // A full implementation would need:
        // 1. Fetch global account data
        // 2. Get fee_recipient pubkey from data
        // 3. get_associated_token_address(&fee_recipient_pubkey, &sol_mint)
        let (fee_recipient_pda,_) = Pubkey::find_program_address(&[FEE_RECIPIENT_SEED], &program_id);


        let (bonding_curve, _) = Pubkey::find_program_address(&[BONDING_CURVE_SEED, mint.as_ref()], &program_id);
        
        let user = self._signer.pubkey();
        let associated_user = get_associated_token_address(&user, &mint);
        
        // This seems to be the SOL vault of the bonding curve.
        let associated_bonding_curve = get_associated_token_address(&bonding_curve, &sol_mint);


        let accounts = vec![
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(fee_recipient_pda, false), // This is likely incorrect, see comment above
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(associated_user, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(token_program_id, false),
            AccountMeta::new_readonly(rent::id(), false),
        ];

        // Discriminator for buy is `213, 16, 22, 9, 85, 20, 158, 200`
        let discriminator: [u8; 8] = [213, 16, 22, 9, 85, 20, 158, 200];
        let instruction_data = BuyInstructionData {
            amount: token_amount,
            max_sol_cost,
        };
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&instruction_data.try_to_vec()?);
        
        Ok(Instruction {
            program_id,
            accounts,
            data,
        })
    }
}

#[async_trait]
impl ExecutionGateway for PumpConnector {
    async fn send_order(&self, order: Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        if order.side != OrderSide::Buy {
            return Err("Only buy orders are supported for Pump.fun".into());
        }

        let mint_str = order.symbol;
        let token_amount = (order.amount * 1_000_000.0) as u64; // Assuming 6 decimal places
        let max_sol_cost = (order.price.unwrap_or(0.0) * 1_000_000_000.0) as u64;

        if max_sol_cost == 0 {
            return Err("Price must be set for buy orders to calculate max_sol_cost".into());
        }

        let instruction = self.create_buy_instruction(&mint_str, token_amount, max_sol_cost)?;

        let recent_blockhash = self._rpc_client.get_latest_blockhash().await?;

        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&self._signer.pubkey()),
        );

        transaction.sign(&[&self._signer], recent_blockhash);

        let signature = self
            ._rpc_client
            .send_and_confirm_transaction(&transaction)
            .await?;

        Ok(signature.to_string())
    }
} 