use crate::{
    config::MevStrategyConfig,
    execution::ExecutionGateway,
    models::{MarketDataSource, Order, OrderSide, Tick},
    strategies::Strategy,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;

/// A simple triangular arbitrage MEV strategy.
///
/// This strategy monitors ticks for three assets (e.g., BTC, ETH, USD)
/// and attempts to find a profitable triangular arbitrage opportunity.
///
/// The pairs are:
/// 1. A/B (e.g., ETH/BTC)
/// 2. B/C (e.g., BTC/USD)
/// 3. C/A (e.g., USD/ETH -> which we invert to ETH/USD)
#[allow(dead_code)]
pub struct MevStrategy {
    execution_gw: Arc<dyn ExecutionGateway>,
    prices: Arc<Mutex<HashMap<String, f64>>>,
    // Configurable parameters
    asset_a: String, // e.g., "ETH"
    asset_b: String, // e.g., "BTC"
    asset_c: String, // e.g., "USDT"
    pair_ab: String, // e.g., "ETHBTC"
    pair_bc: String, // e.g., "BTCUSDT"
    pair_ca: String, // e.g., "ETHUSDT"
    trade_amount_b: f64, // The amount of asset B to start the arbitrage with
    min_profit_threshold: f64, // Minimum profit percentage to execute a trade
}

#[allow(dead_code)]
impl MevStrategy {
    pub fn new(
        execution_gw: Arc<dyn ExecutionGateway>,
        config: &MevStrategyConfig,
    ) -> Self {
        let pair_ab = format!("{}{}", config.asset_a, config.asset_b);
        let pair_bc = format!("{}{}", config.asset_b, config.asset_c);
        let pair_ca = format!("{}{}", config.asset_a, config.asset_c);
        Self {
            execution_gw,
            prices: Arc::new(Mutex::new(HashMap::new())),
            asset_a: config.asset_a.clone(),
            asset_b: config.asset_b.clone(),
            asset_c: config.asset_c.clone(),
            pair_ab,
            pair_bc,
            pair_ca,
            trade_amount_b: config.trade_amount_b,
            min_profit_threshold: config.min_profit_threshold,
        }
    }

    async fn check_arbitrage(&self, tick: &Tick) -> Result<()> {
        let prices = self.prices.lock().await;

        let price_ab = if let Some(p) = prices.get(&self.pair_ab) { *p } else { return Ok(()) };
        let price_bc = if let Some(p) = prices.get(&self.pair_bc) { *p } else { return Ok(()) };
        let price_ca = if let Some(p) = prices.get(&self.pair_ca) { *p } else { return Ok(()) };
        
        // We start with asset B, buy asset A, sell asset A for C, then sell C for B
        // 1. Buy A with B: amount_a = trade_amount_b / price_ab (e.g., ETH bought with BTC)
        //    Here, price_ab is the price of A in terms of B (ETH/BTC price)
        let amount_a = self.trade_amount_b / price_ab;

        // 2. Sell A for C: amount_c = amount_a * price_ca (e.g., ETH sold for USDT)
        let amount_c = amount_a * price_ca;

        // 3. Sell C for B: final_amount_b = amount_c / price_bc (e.g., USDT sold for BTC)
        let final_amount_b = amount_c / price_bc;

        let profit = (final_amount_b - self.trade_amount_b) / self.trade_amount_b;

        if profit > self.min_profit_threshold {
            println!(
                "MEV Opportunity Found! Profit: {:.5}%. Prices: AB: {}, BC: {}, CA: {}",
                profit * 100.0, price_ab, price_bc, price_ca
            );

            // Create orders
            let order1 = Order::market(
                self.pair_ab.clone(),
                OrderSide::Buy,
                amount_a,
                MarketDataSource::Binance,
                Some(Box::new(tick.clone())),
            );
            let order2 = Order::market(
                self.pair_ca.clone(),
                OrderSide::Sell,
                amount_a,
                MarketDataSource::Binance,
                None,
            );
            let order3 = Order::market(
                self.pair_bc.clone(),
                OrderSide::Sell,
                amount_c,
                MarketDataSource::Binance,
                None,
            );

            // Execute orders in sequence
            // In a real scenario, you'd need to handle partial fills and execution delays
            let gw = self.execution_gw.clone();
            tokio::spawn(async move {
                if let Err(e) = gw.send_order(order1).await {
                    eprintln!("MEV order 1 failed: {}", e);
                    return;
                }
                if let Err(e) = gw.send_order(order2).await {
                    eprintln!("MEV order 2 failed: {}", e);
                     return;
                }
                if let Err(e) = gw.send_order(order3).await {
                    eprintln!("MEV order 3 failed: {}", e);
                }
            });
        }
        
        Ok(())
    }
}

#[async_trait]
impl Strategy for MevStrategy {
    async fn on_tick(&mut self, tick: &Tick) -> Result<()> {
        let symbol = tick.symbol.to_uppercase().replace("/", "");
        {
            let mut prices = self.prices.lock().await;
            prices.insert(symbol, tick.price);
        }
        self.check_arbitrage(tick).await?;
        Ok(())
    }
} 