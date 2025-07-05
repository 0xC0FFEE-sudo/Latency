use std::error::Error;
use std::sync::Arc;
use csv;
use latency_x_core::models::Tick;
use latency_x_core::strategies::{arbitrage::Arbitrage, Strategy};
use latency_x_core::backtest::gateway::BacktestExecutionGateway;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Backtester Initializing ---");

    // 1. Load data
    let mut ticks = Vec::new();
    let mut rdr = csv::Reader::from_path("data/ticks.csv")?;
    for result in rdr.records() {
        let record = result?;
        if record.as_slice().is_empty() || (record.len() == 1 && record.get(0).unwrap_or("").is_empty()) {
            continue;
        }
        let tick: Tick = record.deserialize(None)?;
        ticks.push(tick);
    }
    println!("Loaded {} ticks from CSV.", ticks.len());

    // 2. Set up strategy and mock gateways
    let gateway1 = Arc::new(BacktestExecutionGateway::new());
    let gateway2 = Arc::new(BacktestExecutionGateway::new());
    
    let mut strategy = Arbitrage::new(gateway1.clone(), gateway2.clone(), 0.5, 1.0);
    println!("Initialized Arbitrage strategy.");

    // 3. Run backtest
    println!("--- Running Backtest ---");
    for tick in ticks {
        if let Err(e) = strategy.on_tick(&tick).await {
            eprintln!("Strategy error on tick {:?}: {}", tick, e);
        }
    }
    println!("--- Backtest Finished ---");

    // 4. Report results
    let orders1 = gateway1.orders.lock().unwrap();
    let orders2 = gateway2.orders.lock().unwrap();
    println!("\n--- Backtest Report ---");
    println!("Total orders on gateway 1: {}", orders1.len());
    for order in orders1.iter() {
        println!("  - {:?}", order);
    }
    println!("Total orders on gateway 2: {}", orders2.len());
    for order in orders2.iter() {
        println!("  - {:?}", order);
    }
    
    // A real backtester would calculate PnL, Sharpe ratio, etc. here.
    println!("\n(Note: PnL calculation not yet implemented)");

    Ok(())
} 