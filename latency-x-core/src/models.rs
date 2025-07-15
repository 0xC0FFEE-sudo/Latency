use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum MarketDataSource {
    Binance,
    Kraken,
    Coinbase,
    PumpFun,
    Strategy,
}

impl std::fmt::Display for MarketDataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketDataSource::Binance => write!(f, "Binance"),
            MarketDataSource::Kraken => write!(f, "Kraken"),
            MarketDataSource::Coinbase => write!(f, "Coinbase"),
            MarketDataSource::PumpFun => write!(f, "Pump.fun"),
            MarketDataSource::Strategy => write!(f, "Strategy"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub source: MarketDataSource,
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum OrderStatus {
    New,
    Filled,
    Canceled,
    Failed,
}

#[derive(Display, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Display, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub amount: f64,
    pub price: Option<f64>,
    pub status: OrderStatus,
    pub source: MarketDataSource,
    pub created_at: DateTime<Utc>,
    pub triggering_tick: Option<Box<Tick>>,
}

impl Order {
    pub fn market(
        symbol: String,
        side: OrderSide,
        amount: f64,
        source: MarketDataSource,
        triggering_tick: Option<Box<Tick>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            symbol,
            side,
            order_type: OrderType::Market,
            amount,
            price: None,
            status: OrderStatus::New,
            source,
            created_at: Utc::now(),
            triggering_tick,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub order_id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub amount: f64,
    pub price: f64,
    pub source: MarketDataSource,
    pub executed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub order_id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub price: f64,
    pub quantity: f64,
    pub source: MarketDataSource,
    pub executed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Latency {
    pub exchange: MarketDataSource,
    pub latency_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub asset: String,
    pub free: f64,
    pub locked: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub amount: f64,
    pub entry_price: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub target: String,
} 