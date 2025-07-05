use crate::connectors::Connector;
use crate::execution::ExecutionGateway;
use crate::models::{MarketDataSource, Order, Tick};
use anyhow::Result;
use async_trait::async_trait;
use futures_util::stream::StreamExt;
use hmac_sha256;
use hex;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use url::Url;
use std::error::Error;
use backoff::ExponentialBackoff;
use backoff::future::retry;
use chrono;
use tracing::{info, warn, error};

const BINANCE_API_KEY: &str = "YOUR_API_KEY";
const BINANCE_API_SECRET: &str = "YOUR_API_SECRET";
const BINANCE_API_URL: &str = "https://api.binance.com";

pub struct BinanceConnector {
    http_client: Client,
}

impl BinanceConnector {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    fn sign_request(&self, params: &str) -> String {
        let key = BINANCE_API_SECRET.as_bytes();
        let signature = hmac_sha256::HMAC::mac(params.as_bytes(), key);
        hex::encode(signature)
    }
}

#[derive(Debug, Deserialize)]
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

            let (ws_stream, _) = connect_async(url).await.map_err(|e| backoff::Error::transient(e.into()))?;
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
                    timestamp_ms: trade_data.data.timestamp,
                    price: trade_data.data.price.parse().map_err(|e: std::num::ParseFloatError| backoff::Error::permanent(e.into()))?,
                    volume: trade_data.data.quantity.parse().map_err(|e: std::num::ParseFloatError| backoff::Error::permanent(e.into()))?,
                };

                if let Err(e) = sender.send(tick).await {
                    error!("Failed to send tick: {}", e);
                    break;
                }
            }

            Ok(())
        };

        retry(ExponentialBackoff::default(), operation).await
    }
}

#[async_trait]
impl ExecutionGateway for BinanceConnector {
    async fn send_order(&self, order: Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        let endpoint = "/api/v3/order";
        let url = format!("{}{}", BINANCE_API_URL, endpoint);

        let mut params = format!(
            "symbol={}&side={:?}&type={:?}&quantity={}",
            order.symbol, order.side, order.order_type, order.quantity
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

        Ok("mock_binance_order_id".to_string())
    }
} 