pub mod models;
pub mod connectors;
pub mod execution;
pub mod strategies;
pub mod config;
pub mod settlement;
pub mod backtest;

pub fn add(left: usize, right: usize) -> usize {
    left + right
} 