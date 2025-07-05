use crate::connectors::Connector;
use crate::execution::ExecutionGateway;
use crate::models::{MarketDataSource, Order, Tick};
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
use tokio_tungstenite::connect_async;
use url::Url;
use crate::config::KrakenConfig;
use backoff::ExponentialBackoff;
use backoff::future::retry;


const KRAKEN_WSS_URL: &str = "wss://ws.kraken.com/";

pub struct KrakenConnector {
    api_key: String,
    api_secret: String,
}

impl KrakenConnector {
    pub fn new(config: &KrakenConfig) -> Self {
        Self {
            api_key: config.api_key.clone(),
            api_secret: config.api_secret.clone(),
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
            let (ws_stream, _) = connect_async(url).await.map_err(|e| backoff::Error::transient(e.into()))?;
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
                                timestamp_ms: (timestamp * 1000.0) as u64,
                            };

                            if let Err(e) = sender.send(tick).await {
                                eprintln!("[KRAKEN] Failed to send tick: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
            Ok(())
        };

        retry(ExponentialBackoff::default(), operation).await
    }
}

#[async_trait]
impl ExecutionGateway for KrakenConnector {
    async fn send_order(&self, order: Order) -> Result<String, Box<dyn Error + Send + Sync>> {
        let client = reqwest::Client::new();
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis()
            .to_string();

        let mut params = vec![
            ("nonce", nonce.clone()),
            ("ordertype", order.order_type.to_string()),
            ("type", order.side.to_string().to_lowercase()),
            ("volume", order.quantity.to_string()),
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

        if let Some(txid) = response_json["result"]["txid"].as_array() {
            if !txid.is_empty() {
                return Ok(txid[0].as_str().unwrap_or_default().to_string());
            }
        }

        Err("Could not extract transaction ID from Kraken response".into())
    }
} 