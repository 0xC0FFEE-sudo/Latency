pub mod connectors;
pub mod execution;
pub mod models;
pub mod settlement;
pub mod strategies;
pub mod config;
pub mod risk;
pub mod persistence;
pub mod dashboard;

pub use models::{Tick, Order};

pub fn add(left: usize, right: usize) -> usize {
    left + right
} 