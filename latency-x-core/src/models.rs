use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarketDataSource {
    Binance,
    Kraken,
    Coinbase,
    Bullx,
    Pump,
    PumpFun,
    Axiom,
    Photon,
    UniswapV3,
}

impl ToString for MarketDataSource {
    fn to_string(&self) -> String {
        match self {
            MarketDataSource::Binance => "Binance".to_string(),
            MarketDataSource::Kraken => "Kraken".to_string(),
            MarketDataSource::Pump => "Pump".to_string(),
            MarketDataSource::PumpFun => "PumpFun".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tick {
    pub source: MarketDataSource,
    pub symbol: String,
    pub timestamp_ms: u64,
    pub price: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: String,
    pub source: MarketDataSource,
    pub source_address: String,
    pub destination_address: String,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub quantity: f64,
    pub status: OrderStatus,
    pub timestamp_ms: u64,
}

impl Order {
    pub fn market(symbol: &str, quantity: f64, side: OrderSide, source: MarketDataSource, source_address: String, destination_address: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source,
            source_address,
            destination_address,
            symbol: symbol.to_string(),
            side,
            order_type: OrderType::Market,
            price: None, // Market orders don't have a specific price
            quantity,
            status: OrderStatus::New,
            timestamp_ms: Utc::now().timestamp_millis() as u64,
        }
    }
} 