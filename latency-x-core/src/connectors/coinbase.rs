use crate::{
    config::CoinbaseConfig,
    connectors::Connector,
    execution::ExecutionGateway,
    models::{Fill, MarketDataSource, Order, Tick},
    settlement::Settlement,
};
use anyhow::Result;
use async_trait::async_trait;
use cbadv::websocket::{Channel, Client, Message, Product};
use cbadv::RestClient;
use futures_util::stream::StreamExt;
use std::sync::Arc;
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use chrono::Utc;
use crate::dashboard::events::DashboardEvent;
use tokio::sync::broadcast;
use tracing::warn;
use cbadv::rest::types::{OrderSide as CoinbaseOrderSide, CreateOrder, ProductId};
use uuid::Uuid;

pub struct CoinbaseConnector {
    _rest_client: RestClient,
    settlement: Arc<dyn Settlement>,
    fill_sender: Option<Sender<Fill>>,
    dashboard_tx: broadcast::Sender<DashboardEvent>,
}

impl CoinbaseConnector {
    pub fn new(
        config: &CoinbaseConfig,
        settlement: Arc<dyn Settlement>,
        fill_sender: Option<Sender<Fill>>,
        dashboard_tx: broadcast::Sender<DashboardEvent>,
    ) -> Self {
        Self {
            _rest_client: RestClient::new(&config.api_key, &config.api_secret),
            settlement,
            fill_sender,
            dashboard_tx,
        }
    }
}

#[async_trait]
impl Connector for CoinbaseConnector {
    async fn subscribe(
        self: Arc<Self>,
        symbols: &[String],
        sender: mpsc::Sender<Tick>,
    ) -> Result<()> {
        let mut client = Client::new(true).await?;
        let products: Vec<Product> = symbols.iter().map(|s| s.into()).collect();
        client.subscribe(&[Channel::Ticker], &products).await?;

        while let Some(message) = client.listen().await? {
            match message {
                Message::Ticker(ticker) => {
                    let tick = Tick {
                        source: "Coinbase".to_string(),
                        symbol: ticker.product_id,
                        price: ticker.price.parse()?,
                        volume: ticker.volume_24h.parse()?,
                        received_at: Utc::now(),
                    };
                    if sender.send(tick.clone()).await.is_err() {
                        break;
                    }
                    let _ = self.dashboard_tx.send(DashboardEvent::Tick(tick));
                }
                Message::Error(err) => {
                    tracing::error!("Coinbase error: {}", err.message);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ExecutionGateway for CoinbaseConnector {
    async fn send_order(&self, order: Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        let side = match order.side {
            crate::models::OrderSide::Buy => CoinbaseOrderSide::Buy,
            crate::models::OrderSide::Sell => CoinbaseOrderSide::Sell,
        };

        let client_order_id = Uuid::new_v4().to_string();
        let product_id: ProductId = order.symbol.parse()?;

        let new_order = CreateOrder {
            client_order_id,
            product_id,
            side,
            order_type: order.order_type.to_string(),
            product_type: None,
            order_price: order.price.map(|p| p.to_string()),
            size: Some(order.amount.to_string()),
            time_in_force: None,
            cancel_after: None,
            post_only: None,
            self_trade_prevention_id: None,
        };

        let result = self._rest_client.create_order(&new_order).await?;

        if result.success {
            Ok(result.success_response.unwrap().order_id)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                result.error_response.unwrap().message,
            )))
        }
    }
} 