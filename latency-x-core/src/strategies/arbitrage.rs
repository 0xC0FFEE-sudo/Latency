use crate::execution::ExecutionGateway;
use crate::models::{MarketDataSource, Order, OrderSide, Tick, OrderType, OrderStatus};
use crate::strategies::Strategy;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use metrics::{counter, gauge};
use crate::persistence::db::DatabaseManager;

pub struct Arbitrage<E1, E2>
where
    E1: ExecutionGateway + Send + Sync + ?Sized + 'static,
    E2: ExecutionGateway + Send + Sync + ?Sized + 'static,
{
    exchange1: Arc<E1>,
    exchange2: Arc<E2>,
    last_tick1: Arc<Mutex<Option<Tick>>>,
    last_tick2: Arc<Mutex<Option<Tick>>>,
    min_spread: f64,
    quantity: f64,
    db_manager: Arc<DatabaseManager>,
}

impl<E1, E2> Arbitrage<E1, E2>
where
    E1: ExecutionGateway + Send + Sync + ?Sized + 'static,
    E2: ExecutionGateway + Send + Sync + ?Sized + 'static,
{
    pub fn new(exchange1: Arc<E1>, exchange2: Arc<E2>, min_spread: f64, quantity: f64, db_manager: Arc<DatabaseManager>) -> Self {
        Self {
            exchange1,
            exchange2,
            last_tick1: Arc::new(Mutex::new(None)),
            last_tick2: Arc::new(Mutex::new(None)),
            min_spread,
            quantity,
            db_manager,
        }
    }
}

#[async_trait]
impl<E1, E2> Strategy for Arbitrage<E1, E2>
where
    E1: ExecutionGateway + Send + Sync + ?Sized + 'static,
    E2: ExecutionGateway + Send + Sync + ?Sized + 'static,
{
    async fn on_tick(&mut self, tick: &Tick) -> Result<()> {
        let mut last_tick1_guard = self.last_tick1.lock().await;
        let mut last_tick2_guard = self.last_tick2.lock().await;

        match tick.source {
            MarketDataSource::Binance => *last_tick1_guard = Some(tick.clone()),
            MarketDataSource::Kraken => *last_tick2_guard = Some(tick.clone()),
            _ => (),
        }

        if let (Some(tick1), Some(tick2)) = (&*last_tick1_guard, &*last_tick2_guard) {
            let spread = tick2.price - tick1.price;
            gauge!("arbitrage_spread", "symbol" => tick1.symbol.clone()).set(spread);

            if spread.abs() > self.min_spread {
                info!("[ARBITRAGE] Found opportunity! Spread: {}", spread);
                counter!("arbitrage_opportunities", "symbol" => tick1.symbol.clone()).increment(1);

                if spread > 0.0 {
                    // Buy on exchange1, sell on exchange2
                    let buy_order = Order {
                        id: Default::default(),
                        symbol: tick1.symbol.clone(),
                        side: OrderSide::Buy,
                        order_type: OrderType::Market,
                        amount: self.quantity,
                        price: None,
                        status: OrderStatus::New,
                        source: MarketDataSource::Strategy,
                        created_at: Default::default(),
                        triggering_tick: Some(Box::new(tick.clone())),
                    };

                    let sell_order = Order {
                        id: Default::default(),
                        symbol: tick2.symbol.clone(),
                        side: OrderSide::Sell,
                        order_type: OrderType::Market,
                        amount: self.quantity,
                        price: None,
                        status: OrderStatus::New,
                        source: MarketDataSource::Strategy,
                        created_at: Default::default(),
                        triggering_tick: Some(Box::new(tick.clone())),
                    };

                    self.db_manager.save_order(&buy_order).await?;
                    self.db_manager.save_order(&sell_order).await?;
                    self.exchange1.send_order(buy_order).await.map_err(|e| anyhow::anyhow!(e))?;
                    self.exchange2.send_order(sell_order).await.map_err(|e| anyhow::anyhow!(e))?;

                } else {
                    // Buy on exchange2, sell on exchange1
                    let buy_order = Order {
                        id: Default::default(),
                        symbol: tick2.symbol.clone(),
                        side: OrderSide::Buy,
                        order_type: OrderType::Market,
                        amount: self.quantity,
                        price: None,
                        status: OrderStatus::New,
                        source: MarketDataSource::Strategy,
                        created_at: Default::default(),
                        triggering_tick: Some(Box::new(tick.clone())),
                    };

                    let sell_order = Order {
                        id: Default::default(),
                        symbol: tick1.symbol.clone(),
                        side: OrderSide::Sell,
                        order_type: OrderType::Market,
                        amount: self.quantity,
                        price: None,
                        status: OrderStatus::New,
                        source: MarketDataSource::Strategy,
                        created_at: Default::default(),
                        triggering_tick: Some(Box::new(tick.clone())),
                    };
                    self.db_manager.save_order(&buy_order).await?;
                    self.db_manager.save_order(&sell_order).await?;
                    self.exchange2.send_order(buy_order).await.map_err(|e| anyhow::anyhow!(e))?;
                    self.exchange1.send_order(sell_order).await.map_err(|e| anyhow::anyhow!(e))?;
                }
            }
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
    async fn test_arbitrage_creates_orders() {
        // Arrange
        let mut mock_execution_gateway1 = MockExecutionGateway::new();
        let mut mock_execution_gateway2 = MockExecutionGateway::new();
        let db_manager = Arc::new(DatabaseManager::new(":memory:").await.unwrap());
        db_manager.init().await.unwrap();

        mock_execution_gateway1.expect_send_order()
            .withf(|order| order.side == OrderSide::Buy && order.price == Some(50000.0))
            .returning(|_| Ok("order1".to_string()))
            .times(1);
        
        mock_execution_gateway2.expect_send_order()
            .withf(|order| order.side == OrderSide::Sell && order.price == Some(50200.0))
            .returning(|_| Ok("order2".to_string()))
            .times(1);

        let mut strategy = Arbitrage::new(Arc::new(mock_execution_gateway1), Arc::new(mock_execution_gateway2), 100.0, 1.0, db_manager);

        let tick1 = Tick {
            source: MarketDataSource::Binance,
            symbol: "BTC/USD".to_string(),
            price: 50000.0,
            volume: 1.0,
            received_at: 0,
        };

        let tick2 = Tick {
            source: MarketDataSource::Kraken,
            symbol: "BTC/USD".to_string(),
            price: 50200.0,
            volume: 1.0,
            received_at: 0,
        };

        // Act
        strategy.on_tick(&tick1).await.unwrap();
        strategy.on_tick(&tick2).await.unwrap();

        // Assert
        // Mocks handle assertions
    }
} 