use crate::config::HeliusConfig;
use crate::models::Order;
use crate::settlement::Settlement;
use anyhow::Result;
use async_trait::async_trait;
use helius_sdk::{Cluster, Helius};
use std::error::Error;
use std::sync::Arc;

pub struct HeliusSettlement {
    _helius: Arc<Helius>,
}

impl HeliusSettlement {
    // TODO: Make cluster configurable
    pub fn new(config: &HeliusConfig) -> Result<Self> {
        let helius = Arc::new(Helius::new(
            config.api_key.clone(),
            Cluster::MainnetBeta,
        ));
        Ok(Self { _helius: helius })
    }
}

#[async_trait]
impl Settlement for HeliusSettlement {
    async fn send_order(&self, _order: &Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        // TODO: Implement actual transaction sending logic with Helius
        println!("Settling order on Helius (mock)");
        Ok("mock_helius_tx_id".to_string())
    }
} 