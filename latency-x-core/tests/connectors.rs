use std::sync::Arc;
use tokio::sync::{mpsc, broadcast};
use latency_x_core::connectors::{binance::BinanceConnector, Connector};
use latency_x_core::config::Config;
use latency_x_core::connectors::kraken::KrakenConnector;
use latency_x_core::settlement::Settlement;
use latency_x_core::persistence::db::DatabaseManager;
use latency_x_core::models::{Fill, Order};
use mockall::mock;

mock! {
    pub Settlement {}

    impl Settlement for Settlement {
        fn new(config: &latency_x_core::config::HeliusConfig) -> Self;
        async fn get_priority_fee(&self, last_n_blocks: u64, account: &str) -> anyhow::Result<f64>;
        async fn settle(&self, order: &Order) -> anyhow::Result<String>;
    }
}

mock! {
    pub DatabaseManager {
        pub async fn new(url: &str) -> anyhow::Result<Self>;
        pub async fn init(&self) -> anyhow::Result<()>;
        pub async fn save_fill(&self, fill: &Fill) -> anyhow::Result<()>;
    }
}

#[tokio::test]
#[ignore] // This is an integration test and requires a live connection to Binance.
async fn test_binance_connector_subscribe() {
    let (tx, mut rx) = mpsc::channel(32);
    let (fill_tx, _) = mpsc::channel(32);
    let (dashboard_tx, _) = broadcast::channel(32);
    let mock_settlement = Arc::new(MockSettlement::new());
    let mock_db = Arc::new(MockDatabaseManager::new());

    let connector = Arc::new(BinanceConnector::new(mock_settlement, Some(fill_tx), dashboard_tx, mock_db));
    let symbols = vec!["btcusdt".to_string()];

    let subscribe_task = tokio::spawn(async move {
        connector.subscribe(&symbols, tx).await
    });

    // Wait for the first tick
    let result = tokio::time::timeout(std::time::Duration::from_secs(10), rx.recv()).await;

    // Clean up the task
    subscribe_task.abort();

    assert!(result.is_ok(), "Did not receive a tick from Binance within 10 seconds.");
    let tick = result.unwrap().unwrap();
    assert_eq!(tick.symbol.to_lowercase(), "btcusdt");
    assert!(tick.price > 0.0);
}

#[tokio::test]
#[ignore] // This is an integration test and requires a live connection to Kraken.
async fn test_kraken_connector_subscribe() {
    // Note: This test requires a valid `Config.toml` with Kraken credentials.
    let config = Config::from_file("../Config.toml").expect("Failed to load config for test");
    
    let (tx, mut rx) = mpsc::channel(32);
    let (fill_tx, _) = mpsc::channel(32);
    let (dashboard_tx, _) = broadcast::channel(32);
    let mock_settlement = Arc::new(MockSettlement::new());
    let mock_db = Arc::new(MockDatabaseManager::new());

    let connector = Arc::new(KrakenConnector::new(&config.kraken, mock_settlement, Some(fill_tx), dashboard_tx, mock_db));
    let symbols = vec!["BTC/USD".to_string()];

    let subscribe_task = tokio::spawn(async move {
        connector.subscribe(&symbols, tx).await
    });

    // Wait for the first tick
    let result = tokio::time::timeout(std::time::Duration::from_secs(10), rx.recv()).await;

    // Clean up the task
    subscribe_task.abort();

    assert!(result.is_ok(), "Did not receive a tick from Kraken within 10 seconds.");
    let tick = result.unwrap().unwrap();
    assert_eq!(tick.symbol.to_uppercase(), "BTC/USD");
    assert!(tick.price > 0.0);
} 