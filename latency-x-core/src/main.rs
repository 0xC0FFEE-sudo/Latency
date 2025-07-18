mod config;
mod connectors;
mod dashboard;
mod execution;
mod models;
mod persistence;
mod risk;
mod settlement;
mod strategies;

use crate::connectors::{binance::BinanceConnector, kraken::KrakenConnector};
use crate::connectors::Connector;
use crate::execution::ExecutionGateway;
use crate::models::Fill;
use crate::persistence::db::DatabaseManager;
use crate::risk::RiskManager;
use crate::settlement::{helius::HeliusSettlement, Settlement};
use crate::strategies::arbitrage::Arbitrage;
use crate::strategies::market_maker::MarketMaker;
use crate::strategies::Strategy;
use crate::config::Config;
use crate::dashboard::server::start_dashboard_server;
use crate::dashboard::broadcaster_layer::DashboardBroadcastLayer;
use crate::strategies::mev::MevStrategy;
use crate::dashboard::events::DashboardEvent;
use std::sync::Arc;
use tokio::sync::mpsc;
use clap::Parser;
use tracing_subscriber::{self, EnvFilter};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tokio::sync::broadcast;
use tracing_subscriber::util::SubscriberInitExt;

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
    Mev,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::from_file("Config.toml")?;
    let (dashboard_tx, _) = broadcast::channel::<DashboardEvent>(1024);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,latency_x_core=trace"));
    let broadcast_layer = DashboardBroadcastLayer::new(dashboard_tx.clone());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .with(broadcast_layer)
        .init();

    let builder = PrometheusBuilder::new();
    let addr: SocketAddr = ([127, 0, 0, 1], 9000).into();
    builder
        .with_http_listener(addr)
        .install()
        .expect("failed to install Prometheus exporter");

    let core_ids = core_affinity::get_core_ids().unwrap();
    if core_ids.len() < 4 {
        anyhow::bail!("This application requires at least 4 CPU cores to run effectively.");
    }

    let dashboard_tx_clone = dashboard_tx.clone();
    let dashboard_core = core_ids[3];
    let db_manager = Arc::new(DatabaseManager::new("sqlite:latency_x.db").await?);
    db_manager.init().await?;
    let db_manager_for_dashboard = db_manager.clone();
    tokio::spawn(async move {
        core_affinity::set_for_current(dashboard_core);
        start_dashboard_server(dashboard_tx_clone, db_manager_for_dashboard).await;
    });
    let risk_manager = Arc::new(RiskManager::new(db_manager.clone()).await?);

    let (tx, mut rx) = mpsc::channel(1024);
    let (fill_tx, mut fill_rx) = mpsc::channel::<Fill>(1024);

    let settlement: Arc<dyn Settlement> = Arc::new(HeliusSettlement::new(&config.helius, &config.solana)?);

    let binance_connector = Arc::new(BinanceConnector::new(settlement.clone(), Some(fill_tx.clone()), dashboard_tx.clone(), db_manager.clone()));
    let kraken_connector = Arc::new(KrakenConnector::new(&config.kraken, settlement.clone(), Some(fill_tx.clone()), dashboard_tx.clone(), db_manager.clone()));
    let binance_symbols = vec!["btcusdt".to_string()];
    let kraken_symbols = vec!["BTC/USD".to_string()];

    let binance_tx = tx.clone();
    let binance_connector_clone = binance_connector.clone();
    let binance_core = core_ids[0];
    tokio::spawn(async move {
        core_affinity::set_for_current(binance_core);
        if let Err(e) = binance_connector_clone.subscribe(&binance_symbols, binance_tx).await {
            tracing::error!("Binance connector error: {}", e);
        }
    });

    let kraken_tx = tx.clone();
    let kraken_connector_clone = kraken_connector.clone();
    let kraken_core = core_ids[1];
    tokio::spawn(async move {
        core_affinity::set_for_current(kraken_core);
        if let Err(e) = kraken_connector_clone.subscribe(&kraken_symbols, kraken_tx).await {
            tracing::error!("Kraken connector error: {}", e);
        }
    });

    let binance_execution: Arc<dyn ExecutionGateway> = binance_connector.clone();
    let kraken_execution: Arc<dyn ExecutionGateway> = kraken_connector.clone();
    
    let mut strategy: Box<dyn Strategy> = match cli.strategy {
        StrategyChoice::Arbitrage => {
            Box::new(Arbitrage::new(binance_execution, kraken_execution, 0.0001, 1.0, db_manager.clone()))
        }
        StrategyChoice::MarketMaker => {
            Box::new(MarketMaker::new(binance_execution, 0.01, 0.01, "BTCUSDT".to_string(), db_manager.clone()))
        }
        StrategyChoice::Mev => {
            // For now, we'll only use the binance connector for triangular arbitrage.
            let mev_execution: Arc<dyn ExecutionGateway> = Arc::new(BinanceConnector::new(settlement.clone(), Some(fill_tx.clone()), dashboard_tx.clone(), db_manager.clone()));
            Box::new(MevStrategy::new(
                mev_execution,
                &config.mev_strategy,
                db_manager.clone(),
            ))
        }
    };

    // This is a placeholder for the risk manager loop
    let rm_clone = risk_manager.clone();
    let risk_manager_core = core_ids[2];
    tokio::spawn(async move {
        core_affinity::set_for_current(risk_manager_core);
        while let Some(fill) = fill_rx.recv().await {
            rm_clone.on_fill(&fill).await;
        }
    });

    while let Some(tick) = rx.recv().await {
        if let Err(e) = strategy.on_tick(&tick).await {
            tracing::error!("Strategy error: {}", e);
        }
    }

    Ok(())
}
