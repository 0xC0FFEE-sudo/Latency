use crate::models::Order;
use anyhow::Result;
use async_trait::async_trait;

pub mod helius;
 
#[async_trait]
pub trait SettlementHandler: Send + Sync {
    async fn settle(&self, order: &Order) -> Result<String>;
} 