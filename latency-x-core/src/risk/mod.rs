use crate::models::{Fill, OrderSide};
use crate::persistence::PersistenceManager;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RiskManager {
    positions: Mutex<HashMap<String, f64>>,
    persistence: Arc<PersistenceManager>,
}

impl RiskManager {
    pub async fn new(persistence: Arc<PersistenceManager>) -> Result<Self> {
        let positions = persistence.get_positions().await?;
        Ok(Self {
            positions: Mutex::new(positions),
            persistence,
        })
    }

    pub async fn on_fill(&self, fill: &Fill) {
        let positions_clone;
        {
            let mut positions = self.positions.lock().await;
            let position = positions.entry(fill.symbol.clone()).or_insert(0.0);

            match fill.side {
                OrderSide::Buy => *position += fill.quantity,
                OrderSide::Sell => *position -= fill.quantity,
            }

            tracing::info!("Updated position for {}: {}", fill.symbol, *position);
            positions_clone = positions.clone();
        } // Lock is dropped here

        if let Err(e) = self.persistence.set_positions(&positions_clone).await {
            tracing::error!("Failed to save positions: {}", e);
        }
    }
} 