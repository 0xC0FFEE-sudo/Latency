use crate::models::Order;
use crate::settlement::Settlement;
use anyhow::Result;
use async_trait::async_trait;
use std::error::Error;

pub struct MockSettlement;

impl MockSettlement {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Settlement for MockSettlement {
    async fn send_order(&self, order: &Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        println!("Mock settlement for order: {} ({})", order.id, order.symbol);
        Ok(format!("mock_tx_{}", order.id))
    }
}