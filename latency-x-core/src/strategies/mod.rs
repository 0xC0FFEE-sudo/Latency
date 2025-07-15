pub mod arbitrage;
pub mod buy_new_token;
pub mod market_maker;
pub mod mev;


use crate::models::Tick;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Strategy: Send {
    async fn on_tick(&mut self, tick: &Tick) -> Result<()>;
} 