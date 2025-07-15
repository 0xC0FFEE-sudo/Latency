pub mod helius;
pub mod solana;
pub mod mock;

use crate::models::Order;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Settlement: Send + Sync {
    async fn send_order(&self, order: &Order) -> Result<String, Box<dyn Error + Send + Sync>>;
}