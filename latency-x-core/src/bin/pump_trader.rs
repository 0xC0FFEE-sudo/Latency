use latency_x_core::connectors::pump::PumpConnector;
use latency_x_core::strategies::buy_new_token::BuyNewTokenStrategy;
use latency_x_core::config::Config;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Config::from_file("latency-x-core/Config.toml").expect("Failed to load config");
    
    let rpc_client = Arc::new(RpcClient::new(config.solana.rpc_url.clone()));
    let pump_connector = Arc::new(PumpConnector::new(rpc_client.clone(), &config.solana));
    
    let mut strategy = BuyNewTokenStrategy::new(
        pump_connector,
        config.pump_strategy.buy_token_amount,
        config.pump_strategy.max_sol_price_per_token,
    );

    if let Err(e) = strategy.run(&config.solana.ws_url).await {
        eprintln!("Error running BuyNewTokenStrategy: {}", e);
    }
} 