use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub kraken: KrakenConfig,
    // pub helius: HeliusConfig,
    pub helius: HeliusConfig,
    pub solana: SolanaConfig,
    pub pump_strategy: PumpStrategyConfig,
    #[serde(default = "default_mev_strategy_config")]
    pub mev_strategy: MevStrategyConfig,
    #[serde(default = "default_redis_config")]
    pub redis: RedisConfig,
    // #[serde(default = "default_coinbase_config")]
    // pub coinbase: CoinbaseConfig,
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

#[derive(Debug, Deserialize, Clone)]
pub struct MevStrategyConfig {
    pub asset_a: String,
    pub asset_b: String,
    pub asset_c: String,
    pub trade_amount_b: f64,
    pub min_profit_threshold: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

// #[derive(Debug, Deserialize, Clone)]
// pub struct CoinbaseConfig {
//     pub api_key: String,
//     pub api_secret: String,
// }

fn default_mev_strategy_config() -> MevStrategyConfig {
    MevStrategyConfig {
        asset_a: "ETH".to_string(),
        asset_b: "BTC".to_string(),
        asset_c: "USDT".to_string(),
        trade_amount_b: 0.01,
        min_profit_threshold: 0.001,
    }
}

fn default_redis_config() -> RedisConfig {
    RedisConfig {
        url: "redis://127.0.0.1/".to_string(),
    }
}

// fn default_coinbase_config() -> CoinbaseConfig {
//     CoinbaseConfig {
//         api_key: "YOUR_COINBASE_API_KEY".to_string(),
//         api_secret: "YOUR_COINBASE_API_SECRET".to_string(),
//     }
// }

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
        // if config.helius.api_key.starts_with('$') {
        //     config.helius.api_key = env::var(&config.helius.api_key[1..])?;
        // }
        if config.helius.api_key.starts_with('$') {
            config.helius.api_key = env::var(&config.helius.api_key[1..])?;
        }
        if config.solana.private_key.starts_with('$') {
            config.solana.private_key = env::var(&config.solana.private_key[1..])?;
        }
        // if config.coinbase.api_key.starts_with('$') {
        //     config.coinbase.api_key = env::var(&config.coinbase.api_key[1..])?;
        // }
        // if config.coinbase.api_secret.starts_with('$') {
        //     config.coinbase.api_secret = env::var(&config.coinbase.api_secret[1..])?;
        // }

        Ok(config)
    }
} 