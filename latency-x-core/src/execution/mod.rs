#[cfg(test)]
pub mod mock;

use crate::models::Order;
use anyhow::Result;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait ExecutionGateway: Send + Sync {
    async fn send_order(&self, order: Order) -> Result<String, Box<dyn Error + Send + Sync>>;
} 