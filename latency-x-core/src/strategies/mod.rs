pub mod arbitrage;
pub mod buy_new_token;
pub mod market_maker;
pub mod mev;

pub use arbitrage::Arbitrage;
pub use market_maker::MarketMaker;
pub use buy_new_token::BuyNewTokenStrategy;
pub use mev::MevStrategy;

use crate::models::Tick;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Strategy: Send {
    async fn on_tick(&mut self, tick: &Tick) -> Result<()>;
} 