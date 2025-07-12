use crate::execution::ExecutionGateway;
use crate::models::{Order, OrderSide, OrderStatus, Tick, OrderType};
use crate::strategies::Strategy;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use metrics::{counter, gauge};
use chrono::Utc;
use log::error;

pub struct MarketMaker<E: ExecutionGateway + ?Sized> {
    execution_gateway: Arc<E>,
    last_price: Arc<Mutex<Option<f64>>>,
    spread: f64,
    quantity: f64,
    symbol: String,
    order_size: f64,
    ticks: Vec<Tick>,
}

impl<E: ExecutionGateway + Send + Sync + 'static + ?Sized> MarketMaker<E> {
    pub fn new(execution_gateway: Arc<E>, spread: f64, quantity: f64) -> Self {
        Self {
            execution_gateway,
            last_price: Arc::new(Mutex::new(None)),
            spread,
            quantity,
            symbol: "BTCUSDT".to_string(), // Placeholder, will be set later
            order_size: quantity,
            ticks: Vec::new(),
        }
    }
}

#[async_trait]
impl<E: ExecutionGateway + Send + Sync + 'static + ?Sized> Strategy for MarketMaker<E> {
    async fn on_tick(&mut self, tick: &Tick) -> Result<()> {
        let mut last_price_guard = self.last_price.lock().await;
        let last_price = *last_price_guard;
        *last_price_guard = Some(tick.price);

        gauge!("last_price", "symbol" => tick.symbol.clone()).set(tick.price);

        if last_price.is_some() {
            let buy_price = tick.price * (1.0 - self.spread);
            let sell_price = tick.price * (1.0 + self.spread);

            let buy_order = Order {
                id: Uuid::new_v4(),
                symbol: tick.symbol.clone(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                amount: self.quantity,
                price: Some(buy_price),
                status: OrderStatus::New,
                source: tick.source,
                created_at: Utc::now(),
                triggering_tick: Some(Box::new(tick.clone())),
            };

            let sell_order = Order {
                id: Uuid::new_v4(),
                symbol: tick.symbol.clone(),
                side: OrderSide::Sell,
                order_type: OrderType::Limit,
                amount: self.quantity,
                price: Some(sell_price),
                status: OrderStatus::New,
                source: tick.source,
                created_at: Utc::now(),
                triggering_tick: Some(Box::new(tick.clone())),
            };
            
            counter!("orders_created", "strategy" => "market_maker", "side" => "buy").increment(1);
            self.execution_gateway.send_order(buy_order).await.map_err(|e| anyhow::anyhow!(e))?;
            
            counter!("orders_created", "strategy" => "market_maker", "side" => "sell").increment(1);
            self.execution_gateway.send_order(sell_order).await.map_err(|e| anyhow::anyhow!(e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::mock::MockExecutionGateway;
    use crate::models::{MarketDataSource, Tick};

    #[tokio::test]
    async fn test_market_maker_creates_orders() {
        // Arrange
        let mut mock_execution_gateway = MockExecutionGateway::new();
        mock_execution_gateway.expect_send_order()
            .returning(|_| Ok("test_order_id".to_string()))
            .times(2);

        let mut strategy = MarketMaker::new(Arc::new(mock_execution_gateway), 0.01, 1.0);
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