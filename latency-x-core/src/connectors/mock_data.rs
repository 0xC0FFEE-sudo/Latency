use crate::connectors::Connector;
use crate::models::{MarketDataSource, Tick};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

pub struct MockDataConnector {
    source: MarketDataSource,
    base_price: f64,
}

impl MockDataConnector {
    pub fn new(source: MarketDataSource, base_price: f64) -> Self {
        Self { source, base_price }
    }
}

#[async_trait]
impl Connector for MockDataConnector {
    async fn subscribe(
        self: Arc<Self>,
        symbols: &[String],
        sender: mpsc::Sender<Tick>,
    ) -> Result<()> {
        let mut interval = interval(Duration::from_millis(100)); // 10 ticks per second
        let mut rng = StdRng::from_entropy();
        let mut current_price = self.base_price;

        loop {
            interval.tick().await;
            
            // Simulate price movement with random walk
            let change_percent = rng.gen_range(-0.001..0.001); // ±0.1% change
            current_price *= 1.0 + change_percent;
            
            for symbol in symbols {
                let tick = Tick {
                    source: self.source,
                    symbol: symbol.clone(),
                    price: current_price,
                    volume: rng.gen_range(0.1..10.0),
                    received_at: Utc::now(),
                };

                if sender.send(tick).await.is_err() {
                    return Ok(()); // Channel closed, exit gracefully
                }
            }
        }
    }

    fn get_source(&self) -> MarketDataSource {
        self.source
    }
}