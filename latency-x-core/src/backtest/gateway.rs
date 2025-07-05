use async_trait::async_trait;
use std::error::Error;
use std::sync::{Arc, Mutex};
use crate::execution::ExecutionGateway;
use crate::models::Order;

#[derive(Debug, Clone, Default)]
pub struct BacktestExecutionGateway {
    pub orders: Arc<Mutex<Vec<Order>>>,
}

impl BacktestExecutionGateway {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ExecutionGateway for BacktestExecutionGateway {
    async fn send_order(&self, order: Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        let order_id = order.id.clone();
        self.orders.lock().unwrap().push(order);
        println!("[BACKTEST] Executed order: {}", order_id);
        Ok(order_id)
    }
} 