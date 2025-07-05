use latency_x_core::connectors::{binance::BinanceConnector, kraken::KrakenConnector};
use latency_x_core::connectors::Connector;
use latency_x_core::execution::ExecutionGateway;
use latency_x_core::strategies::arbitrage::Arbitrage;
use latency_x_core::strategies::market_maker::MarketMaker;
use latency_x_core::strategies::Strategy;
use latency_x_core::config::Config;
use std::sync::Arc;
use tokio::sync::mpsc;
use clap::Parser;
use tracing_subscriber::{self, EnvFilter};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;

/// A high-frequency trading bot in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The trading strategy to use
    #[arg(short, long, value_enum)]
    strategy: StrategyChoice,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum StrategyChoice {
    Arbitrage,
    MarketMaker,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    
    let builder = PrometheusBuilder::new();
    let addr: SocketAddr = ([127, 0, 0, 1], 9000).into();
    builder
        .with_http_listener(addr)
        .install()
        .expect("failed to install Prometheus exporter");

    tracing::info!(metrics_addr = %addr, "Prometheus exporter listening");
    
    tracing::info!("Starting Latency-X Core");

    let cli = Cli::parse();

    let config = Config::from_file("latency-x-core/Config.toml").expect("Failed to load config");

    let (tx, mut rx) = mpsc::channel(1024);

    let binance_connector = Arc::new(BinanceConnector::new());
    let kraken_connector = Arc::new(KrakenConnector::new(&config.kraken));
    let binance_symbols = vec!["btcusdt".to_string()];
    let kraken_symbols = vec!["BTC/USD".to_string()];

    let binance_tx = tx.clone();
    let binance_connector_clone = binance_connector.clone();
    tokio::spawn(async move {
        if let Err(e) = binance_connector_clone.subscribe(&binance_symbols, binance_tx).await {
            tracing::error!("Binance connector error: {}", e);
        }
    });

    let kraken_tx = tx.clone();
    let kraken_connector_clone = kraken_connector.clone();
    tokio::spawn(async move {
        if let Err(e) = kraken_connector_clone.subscribe(&kraken_symbols, kraken_tx).await {
            tracing::error!("Kraken connector error: {}", e);
        }
    });

    let binance_execution: Arc<dyn ExecutionGateway> = binance_connector.clone();
    let kraken_execution: Arc<dyn ExecutionGateway> = kraken_connector.clone();
    
    let mut strategy: Box<dyn Strategy> = match cli.strategy {
        StrategyChoice::Arbitrage => {
            Box::new(Arbitrage::new(binance_execution, kraken_execution, 0.0001, 1.0))
        }
        StrategyChoice::MarketMaker => {
            Box::new(MarketMaker::new(binance_execution, 0.01, 0.01))
        }
    };

    while let Some(tick) = rx.recv().await {
        if let Err(e) = strategy.on_tick(&tick).await {
            tracing::error!("Strategy error: {}", e);
        }
    }
}
