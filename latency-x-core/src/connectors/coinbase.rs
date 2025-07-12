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
    async fn send_order(&self, _order: Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        // TODO: Implement order sending for Coinbase
        unimplemented!("Coinbase order sending is not yet implemented")
    }
} 