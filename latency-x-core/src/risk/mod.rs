use crate::models::{Fill, OrderSide};
use crate::persistence::db::DatabaseManager;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RiskManager {
    positions: Mutex<HashMap<String, f64>>,
    db_manager: Arc<DatabaseManager>,
}

impl RiskManager {
    pub async fn new(db_manager: Arc<DatabaseManager>) -> Result<Self> {
        let positions = db_manager.get_positions().await?;
        Ok(Self {
            positions: Mutex::new(positions),
            db_manager,
        })
    }

    pub async fn on_fill(&self, fill: &Fill) {
        if let Err(e) = self.db_manager.save_fill(fill).await {
            tracing::error!("Failed to save fill: {}", e);
        }

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

        if let Err(e) = self.db_manager.set_positions(&positions_clone).await {
            tracing::error!("Failed to save positions: {}", e);
        }
    }
} 