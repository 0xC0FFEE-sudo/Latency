use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub kraken: KrakenConfig,
    pub helius: HeliusConfig,
    pub solana: SolanaConfig,
    pub pump_strategy: PumpStrategyConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KrakenConfig {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HeliusConfig {
    pub api_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub private_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PumpStrategyConfig {
    pub buy_token_amount: f64,
    pub max_sol_price_per_token: f64,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;
        
        let mut config: Config = settings.try_deserialize()?;

        if config.kraken.api_key.starts_with('$') {
            config.kraken.api_key = env::var(&config.kraken.api_key[1..])?;
        }
        if config.kraken.api_secret.starts_with('$') {
            config.kraken.api_secret = env::var(&config.kraken.api_secret[1..])?;
        }
        if config.helius.api_key.starts_with('$') {
            config.helius.api_key = env::var(&config.helius.api_key[1..])?;
        }
        if config.solana.private_key.starts_with('$') {
            config.solana.private_key = env::var(&config.solana.private_key[1..])?;
        }

        Ok(config)
    }
} 