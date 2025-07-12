use std::sync::Arc;
use crate::execution::ExecutionGateway;
use std::error::Error;
use solana_pubsub_client::pubsub_client::PubsubClient;
use solana_rpc_client_api::config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use crate::models::{Order, OrderSide, OrderType};
use borsh::BorshDeserialize;
use base64::{engine::general_purpose, Engine as _};
use backoff::ExponentialBackoff;
use backoff::future::retry;
use uuid::Uuid;
use chrono::Utc;
use crate::models::OrderStatus;
use crate::models::MarketDataSource;

const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const CREATE_EVENT_DISCRIMINATOR: [u8; 8] = [84, 97, 40, 193, 72, 143, 120, 163];

#[derive(BorshDeserialize, Debug)]
#[allow(dead_code)]
struct CreateEvent {
    name: String,
    symbol: String,
    uri: String,
    mint: Pubkey,
    bonding_curve: Pubkey,
    user: Pubkey,
}


pub struct BuyNewTokenStrategy {
    execution_gateway: Arc<dyn ExecutionGateway + Send + Sync>,
    buy_token_amount: f64,
    max_sol_price_per_token: f64,
}

impl BuyNewTokenStrategy {
    pub fn new(
        execution_gateway: Arc<dyn ExecutionGateway + Send + Sync>,
        buy_token_amount: f64,
        max_sol_price_per_token: f64,
    ) -> Self {
        Self {
            execution_gateway,
            buy_token_amount,
            max_sol_price_per_token,
        }
    }

    pub async fn run(&mut self, ws_url: &str) -> Result<(), Box<dyn Error>> {
        println!("Connecting to Solana WebSocket for Pump.fun new tokens...");

        let operation = || async {
            let (mut client, receiver) = PubsubClient::logs_subscribe(
                ws_url,
                RpcTransactionLogsFilter::Mentions(vec![PUMP_PROGRAM_ID.to_string()]),
                RpcTransactionLogsConfig {
                    commitment: Some(CommitmentConfig::processed()),
                },
            )
            .map_err(|e| backoff::Error::transient(Box::new(e) as Box<dyn Error + Send + Sync>))?;

            println!("Subscribed to logs for program: {}", PUMP_PROGRAM_ID);

            while let Ok(log) = receiver.recv() {
                for log_message in &log.value.logs {
                    if let Some(data) = log_message.strip_prefix("Program data: ") {
                        if let Ok(decoded_data) = general_purpose::STANDARD.decode(data) {
                            if decoded_data.starts_with(&CREATE_EVENT_DISCRIMINATOR) {
                                if let Ok(event) = CreateEvent::try_from_slice(&decoded_data[8..]) {
                                    println!("New token created: {:?}", event);
                                    println!("Mint: {}", event.mint);

                                    let order = Order {
                                        id: Uuid::new_v4(),
                                        symbol: event.mint.to_string(),
                                        side: OrderSide::Buy,
                                        order_type: OrderType::Limit,
                                        amount: self.buy_token_amount,
                                        price: Some(self.max_sol_price_per_token),
                                        status: OrderStatus::New,
                                        source: MarketDataSource::PumpFun,
                                        created_at: Utc::now(),
                                        triggering_tick: None,
                                    };
                                    
                                    match self.execution_gateway.send_order(order).await {
                                        Ok(order_id) => {
                                            println!("Successfully sent buy order for {}: {}", event.mint, order_id);
                                        }
                                        Err(e) => {
                                            eprintln!("Error sending buy order for {}: {}", event.mint, e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            if let Err(e) = client.shutdown() {
                eprintln!("Error shutting down pubsub client: {:?}", e);
            }
            
            println!("WebSocket connection closed.");
            Ok(())
        };

        retry(ExponentialBackoff::default(), operation)
            .await
            .map_err(|e| -> Box<dyn Error> {
                match e {
                    backoff::Error::Transient { err, .. } => err,
                    backoff::Error::Permanent(err) => err,
                }
            })
    }
} 