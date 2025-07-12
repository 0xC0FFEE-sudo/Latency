use crate::connectors::Connector;
use crate::execution::ExecutionGateway;
use crate::models::{Fill, MarketDataSource, Order, Tick};
use crate::settlement::Settlement;
use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use futures_util::{SinkExt, StreamExt};
use hmac::{Hmac, Mac};
use reqwest;
use serde_json::json;
use sha2::{Digest, Sha256, Sha512};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::connect_async;
use url::Url;
use crate::config::KrakenConfig;
use backoff::ExponentialBackoff;
use backoff::future::retry;
use chrono::Utc;
use tracing::{error, info};
use crate::dashboard::events::DashboardEvent;
use tokio::sync::broadcast;
use tracing::warn;
use uuid::Uuid;


const KRAKEN_WSS_URL: &str = "wss://ws.kraken.com/";

pub struct KrakenConnector {
    api_key: String,
    api_secret: String,
    settlement: Arc<dyn Settlement>,
    fill_sender: Option<Sender<Fill>>,
    dashboard_tx: broadcast::Sender<DashboardEvent>,
}

impl KrakenConnector {
    pub fn new(
        kraken_config: &KrakenConfig,
        settlement: Arc<dyn Settlement>,
        fill_sender: Option<Sender<Fill>>,
        dashboard_tx: broadcast::Sender<DashboardEvent>,
    ) -> Self {
        Self {
            api_key: kraken_config.api_key.clone(),
            api_secret: kraken_config.api_secret.clone(),
            settlement,
            fill_sender,
            dashboard_tx,
        }
    }

    fn get_kraken_signature(
        &self,
        path: &str,
        nonce: &str,
        data: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let secret = general_purpose::STANDARD.decode(&self.api_secret)?;
        let mut mac = Hmac::<Sha512>::new_from_slice(&secret)?;
        let mut message = nonce.to_string();
        message.push_str(data);
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        let sha256_result = hasher.finalize();

        mac.update(path.as_bytes());
        mac.update(&sha256_result);
        let signature = mac.finalize().into_bytes();
        Ok(general_purpose::STANDARD.encode(signature))
    }
}

#[async_trait]
impl Connector for KrakenConnector {
    async fn subscribe(
        self: Arc<Self>,
        symbols: &[String],
        sender: mpsc::Sender<Tick>,
    ) -> Result<()> {
        let operation = || async {
            let url = Url::parse(KRAKEN_WSS_URL).map_err(|e| backoff::Error::transient(e.into()))?;
            let (ws_stream, _) = connect_async(url.as_str()).await.map_err(|e| backoff::Error::transient(e.into()))?;
            let (mut write, mut read) = ws_stream.split();

            let subscribe_msg = json!({
                "event": "subscribe",
                "pair": symbols,
                "subscription": {
                    "name": "trade"
                }
            });

            write.send(tokio_tungstenite::tungstenite::Message::Text(
                subscribe_msg.to_string(),
            ))
            .await.map_err(|e| backoff::Error::transient(e.into()))?;
            println!("[KRAKEN] Subscribed to trades for {:?}", symbols);

            while let Some(Ok(msg)) = read.next().await {
                if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                    let v: serde_json::Value = match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };

                    if let Some(trades) = v.get(1).and_then(|d| d.as_array()) {
                        for trade in trades {
                            let price = trade[0].as_str().unwrap_or("0").parse::<f64>().map_err(|e| backoff::Error::permanent(e.into()))?;
                            let volume = trade[1].as_str().unwrap_or("0").parse::<f64>().map_err(|e| backoff::Error::permanent(e.into()))?;
                            let timestamp = trade[2].as_str().unwrap_or("0").parse::<f64>().map_err(|e| backoff::Error::permanent(e.into()))?;
                            let symbol = v[3].as_str().unwrap_or_default().to_string();

                            let tick = Tick {
                                source: MarketDataSource::Kraken,
                                symbol,
                                price,
                                volume,
                                received_at: Utc::now(),
                            };

                            if let Err(e) = sender.send(tick.clone()).await {
                                eprintln!("[KRAKEN] Failed to send tick: {}", e);
                                break;
                            }
                            let _ = self.dashboard_tx.send(DashboardEvent::Tick(tick));
                        }
                    }
                }
            }
            Ok(())
        };

        retry(ExponentialBackoff::default(), operation).await
    }

    fn get_source(&self) -> MarketDataSource {
        MarketDataSource::Kraken
    }
}

#[async_trait]
impl ExecutionGateway for KrakenConnector {
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

        let client = reqwest::Client::new();
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis()
            .to_string();

        let mut params = vec![
            ("nonce", nonce.clone()),
            ("ordertype", order.order_type.to_string()),
            ("type", order.side.to_string().to_lowercase()),
            ("volume", order.amount.to_string()),
            ("pair", order.symbol.clone()),
        ];
        if let Some(price) = order.price {
            params.push(("price", price.to_string()));
        }

        let body = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        let path = "/0/private/AddOrder".to_string();

        let signature = self.get_kraken_signature(&path, &nonce, &body)?;

        let res = client
            .post("https://api.kraken.com/0/private/AddOrder")
            .header("API-Key", &self.api_key)
            .header("API-Sign", signature)
            .form(&params)
            .send()
            .await?;

        let response_text = res.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(error) = response_json["error"].as_array() {
            if !error.is_empty() {
                return Err(format!("Kraken API Error: {:?}", error).into());
            }
        }

        let price = order.price.unwrap_or_else(|| {
            warn!("Order price is None, using 1.0 as a mock price");
            1.0
        });

        let fill = Fill {
            order_id: order.id,
            symbol: order.symbol.clone(),
            side: order.side,
            price,
            quantity: order.amount,
            source: MarketDataSource::Kraken,
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
            symbol: fill.symbol,
            side: fill.side,
            amount: fill.quantity,
            price: fill.price,
            source: fill.source,
            executed_at: fill.executed_at,
        };

        let _ = self.dashboard_tx.send(DashboardEvent::Trade(trade));

        self.settlement.send_order(&order).await?;

        if let Some(txid) = response_json["result"]["txid"].as_array() {
            if !txid.is_empty() {
                return Ok(txid[0].as_str().unwrap_or_default().to_string());
            }
        }

        Err("Could not extract transaction ID from Kraken response".into())
    }
} 