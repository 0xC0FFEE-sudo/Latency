use crate::execution::ExecutionGateway;
use crate::models::{Order, OrderSide, OrderStatus, Tick, OrderType, MarketDataSource};
use crate::strategies::Strategy;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use metrics::{counter, gauge};
use chrono::Utc;
use crate::persistence::db::DatabaseManager;

pub struct MarketMaker {
    execution_gateway: Arc<dyn ExecutionGateway + Send + Sync>,
    last_price: Arc<Mutex<Option<f64>>>,
    spread: f64,
    quantity: f64,
    symbol: String,
    db_manager: Arc<DatabaseManager>,
}

impl MarketMaker {
    pub fn new(execution_gateway: Arc<dyn ExecutionGateway + Send + Sync>, spread: f64, quantity: f64, symbol: String, db_manager: Arc<DatabaseManager>) -> Self {
        Self {
            execution_gateway,
            last_price: Arc::new(Mutex::new(None)),
            spread,
            quantity,
            symbol,
            db_manager,
        }
    }
}

#[async_trait]
impl Strategy for MarketMaker {
    async fn on_tick(&mut self, tick: &Tick) -> Result<()> {
        let mut last_price = self.last_price.lock().await;
        gauge!("last_price", "symbol" => tick.symbol.clone()).set(tick.price);
        *last_price = Some(tick.price);

        if let Some(price) = *last_price {
            let bid_price = price * (1.0 - self.spread);
            let ask_price = price * (1.0 + self.spread);

            let buy_order = Order {
                id: Uuid::new_v4(),
                symbol: self.symbol.clone(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                amount: self.quantity,
                price: Some(bid_price),
                status: OrderStatus::New,
                source: MarketDataSource::Strategy,
                created_at: Utc::now(),
                triggering_tick: None,
            };

            let sell_order = Order {
                id: Uuid::new_v4(),
                symbol: self.symbol.clone(),
                side: OrderSide::Sell,
                order_type: OrderType::Limit,
                amount: self.quantity,
                price: Some(ask_price),
                status: OrderStatus::New,
                source: MarketDataSource::Strategy,
                created_at: Utc::now(),
                triggering_tick: None,
            };

            self.db_manager.save_order(&buy_order).await?;
            self.db_manager.save_order(&sell_order).await?;
            self.execution_gateway.send_order(buy_order.clone()).await.map_err(|e| anyhow!(e))?;
            counter!("orders_created", "strategy" => "market_maker", "side" => "buy").increment(1);
            
            self.execution_gateway.send_order(sell_order.clone()).await.map_err(|e| anyhow!(e))?;
            counter!("orders_created", "strategy" => "market_maker", "side" => "sell").increment(1);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::mock::MockExecutionGateway;
    use crate::models::{MarketDataSource, Tick};
    use crate::persistence::db::DatabaseManager;

    #[tokio::test]
    async fn test_market_maker_creates_orders() {
        // Arrange
        let mut mock_execution_gateway = MockExecutionGateway::new();
        let db_manager = Arc::new(DatabaseManager::new(":memory:").await.unwrap());
        db_manager.init().await.unwrap();

        mock_execution_gateway.expect_send_order()
            .returning(|_| Ok("test_order_id".to_string()))
            .times(2);

        let mut strategy = MarketMaker::new(Arc::new(mock_execution_gateway), 0.01, 1.0, "BTCUSDT".to_string(), db_manager);
        let tick = Tick {
            source: MarketDataSource::Binance,
            symbol: "BTCUSDT".to_string(),
            price: 50000.0,
            volume: 1.0,
            received_at: Utc::now(),
        };

        // Act
        // First tick just sets the price
        strategy.on_tick(&tick).await.unwrap();
        // Second tick should create orders
        strategy.on_tick(&tick).await.unwrap();

        // Assert
        // The mock expectations handle the assertion
    }
} 