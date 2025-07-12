use crate::config::RedisConfig;
use anyhow::Result;
use redis::{AsyncCommands, Client};
use std::collections::HashMap;

const POSITIONS_KEY: &str = "latency-x:positions";

pub struct PersistenceManager {
    client: Client,
}

impl PersistenceManager {
    pub fn new(config: &RedisConfig) -> Result<Self> {
        let client = Client::open(config.url.as_ref())?;
        Ok(Self { client })
    }

    pub async fn get_positions(&self) -> Result<HashMap<String, f64>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let positions: Option<String> = conn.get(POSITIONS_KEY).await?;
        match positions {
            Some(p) => Ok(serde_json::from_str(&p)?),
            None => Ok(HashMap::new()),
        }
    }

    pub async fn set_positions(&self, positions: &HashMap<String, f64>) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let positions_str = serde_json::to_string(positions)?;
        conn.set::<_, _, ()>(POSITIONS_KEY, positions_str).await?;
        Ok(())
    }
} 