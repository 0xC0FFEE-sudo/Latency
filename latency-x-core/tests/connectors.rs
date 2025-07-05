use std::sync::Arc;
use tokio::sync::mpsc;
use latency_x_core::connectors::{binance::BinanceConnector, Connector};
use latency_x_core::config::Config;
use latency_x_core::connectors::kraken::KrakenConnector;

#[tokio::test]
#[ignore] // This is an integration test and requires a live connection to Binance.
async fn test_binance_connector_subscribe() {
    let (tx, mut rx) = mpsc::channel(32);
    let connector = Arc::new(BinanceConnector::new());
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
    let connector = Arc::new(KrakenConnector::new(&config.kraken));
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