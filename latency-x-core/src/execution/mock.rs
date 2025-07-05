use crate::models::Order;
use async_trait::async_trait;
use std::error::Error;
use super::ExecutionGateway;
use mockall::mock;

mock! {
    pub ExecutionGateway {}

    #[async_trait]
    impl ExecutionGateway for ExecutionGateway {
        async fn send_order(&self, order: Order) -> Result<String, Box<dyn Error + Send + Sync>>;
    }
} 