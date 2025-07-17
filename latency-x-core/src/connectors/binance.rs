use crate::connectors::Connector;
use crate::execution::ExecutionGateway;
use crate::models::{Fill, MarketDataSource, Order, Tick};
use crate::settlement::Settlement;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::stream::StreamExt;
use hmac_sha256;
use hex;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::connect_async;
use url::Url;
use std::error::Error;
use backoff::ExponentialBackoff;
use backoff::future::retry;
use chrono::{self, Utc};
use tracing::{info, warn, error};
use crate::dashboard::events::DashboardEvent;
use tokio::sync::broadcast;
use crate::persistence::db::DatabaseManager;
use uuid::Uuid;

const BINANCE_API_KEY: &str = "YOUR_API_KEY";
const BINANCE_API_SECRET: &str = "YOUR_API_SECRET";
const BINANCE_API_URL: &str = "https://api.binance.com";

pub struct BinanceConnector {
    http_client: Client,
    settlement: Arc<dyn Settlement>,
    fill_sender: Option<Sender<Fill>>,
    dashboard_tx: broadcast::Sender<DashboardEvent>,
    db_manager: Arc<DatabaseManager>,
}

impl BinanceConnector {
    pub fn new(
        settlement: Arc<dyn Settlement>,
        fill_sender: Option<Sender<Fill>>,
        dashboard_tx: broadcast::Sender<DashboardEvent>,
        db_manager: Arc<DatabaseManager>,
    ) -> Self {
        Self {
            http_client: Client::new(),
            settlement,
            fill_sender,
            dashboard_tx,
            db_manager,
        }
    }

    fn sign_request(&self, params: &str) -> String {
        let key = BINANCE_API_SECRET.as_bytes();
        let signature = hmac_sha256::HMAC::mac(params.as_bytes(), key);
        hex::encode(signature)
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BinanceTrade {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "T")]
    timestamp: u64,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
}

#[derive(Debug, Deserialize)]
struct BinanceStreamData {
    data: BinanceTrade,
}

#[async_trait]
impl Connector for BinanceConnector {
    async fn subscribe(
        self: Arc<Self>,
        symbols: &[String],
        sender: mpsc::Sender<Tick>,
    ) -> Result<()> {
        let operation = || async {
            let streams = symbols
                .iter()
                .map(|s| format!("{}@trade", s.to_lowercase()))
                .collect::<Vec<_>>()
                .join("/");
            let url = Url::parse(&format!(
                "wss://stream.binance.com:9443/stream?streams={}",
                streams
            )).map_err(|e| backoff::Error::transient(e.into()))?;

            let (ws_stream, _) = connect_async(url.as_str()).await.map_err(|e| backoff::Error::transient(e.into()))?;
            info!("Connected to Binance WebSocket");

            let (_, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                let data = message.map_err(|e| backoff::Error::transient(e.into()))?;
                let msg_str = data.to_text().map_err(|e| backoff::Error::transient(e.into()))?;
                let trade_data: BinanceStreamData = match serde_json::from_str(msg_str) {
                    Ok(data) => data,
                    Err(e) => {
                        warn!("Failed to parse message: {:?}, error: {}", msg_str, e);
                        continue;
                    }
                };

                let tick = Tick {
                    source: MarketDataSource::Binance,
                    symbol: trade_data.data.symbol.clone(),
                    price: trade_data.data.price.parse().map_err(|e: std::num::ParseFloatError| backoff::Error::permanent(e.into()))?,
                    volume: trade_data.data.quantity.parse().map_err(|e: std::num::ParseFloatError| backoff::Error::permanent(e.into()))?,
                    received_at: Utc::now(),
                };

                if let Err(e) = sender.send(tick.clone()).await {
                    error!("Failed to send tick: {}", e);
                    break;
                }
                let _ = self.dashboard_tx.send(DashboardEvent::Tick(tick));
            }

            Ok(())
        };

        retry(ExponentialBackoff::default(), operation).await
    }

    fn get_source(&self) -> MarketDataSource {
        MarketDataSource::Binance
    }
}

#[async_trait]
impl ExecutionGateway for BinanceConnector {
    async fn send_order(&self, order: Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        let executed_at = Utc::now();

        if let Some(tick) = &order.triggering_tick {
            let latency = executed_at.signed_duration_since(tick.received_at);
            let latency_us = latency.num_microseconds().unwrap_or(-1);
            info!(
                latency_us = latency_us,
                order_id = %order.id,
                "Tick-to-trade latency"
            );
            let _ = self.dashboard_tx.send(DashboardEvent::LatencyUpdate {
                order_id: order.id.clone(),
                latency_us: latency_us as u64,
            });
        }

        info!(
            order_id = %order.id,
            symbol = %order.symbol,
            side = ?order.side,
            amount = %order.amount,
            price = ?order.price,
            "Executing order"
        );

        let endpoint = "/api/v3/order";
        let url = format!("{}{}", BINANCE_API_URL, endpoint);

        let mut params = format!(
            "symbol={}&side={:?}&type={:?}&quantity={}",
            order.symbol, order.side, order.order_type, order.amount
        );

        if let Some(price) = order.price {
            params.push_str(&format!("&price={}", price));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        params.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&params);
        params.push_str(&format!("&signature={}", signature));
        
        info!("Sending order to Binance: {}?{}", url, params);

        let res = self.http_client
            .post(&url)
            .header("X-MBX-APIKEY", BINANCE_API_KEY)
            .body(params)
            .send()
            .await;
        
        info!("Pretending to send order, response: {:?}", res);

        // In a real implementation, you would parse the exchange's response
        // to create the Fill object. Here we'll just create a mock fill.
        let price = order.price.unwrap_or(1.0);
        let fill = Fill {
            order_id: order.id,
            symbol: order.symbol.clone(),
            side: order.side,
            price, // Mock price
            quantity: order.amount,
            source: MarketDataSource::Binance,
            executed_at,
        };

        if let Some(sender) = &self.fill_sender {
            if let Err(e) = sender.send(fill.clone()).await {
                error!("Failed to send fill: {}", e);
            }
        }

        let trade = crate::models::Trade {
            id: Uuid::new_v4(),
            order_id: fill.order_id,
            symbol: fill.symbol.clone(),
            side: fill.side,
            amount: fill.quantity,
            price: fill.price,
            source: fill.source,
            executed_at: fill.executed_at,
        };
        if let Err(e) = self.db_manager.save_trade(&trade).await {
            error!("Failed to save trade to DB: {}", e);
        }
        let _ = self.dashboard_tx.send(DashboardEvent::Trade(trade));

        self.settlement.send_order(&order).await?;

        Ok("mock_binance_order_id".to_string())
    }
} 