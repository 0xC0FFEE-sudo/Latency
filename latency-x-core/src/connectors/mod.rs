pub mod binance;
// pub mod coinbase;
pub mod kraken;
pub mod pump;
pub mod mock_data;

use crate::models::{Tick, MarketDataSource};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc;

#[async_trait]
pub trait Connector: Send + Sync {
    async fn subscribe(
        self: Arc<Self>,
        symbols: &[String],
        sender: mpsc::Sender<Tick>,
    ) -> Result<()>;

    fn get_source(&self) -> MarketDataSource;
} 