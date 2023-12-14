use serde::{Deserialize, Serialize};

use super::env_var;

#[derive(Serialize, Deserialize)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub db: u8,
}

impl RedisSettings {
    pub(super) fn new_from_env() -> Self {
        Self {
            host: env_var("REDIS_HOST"),
            port: env_var("REDIS_PORT").parse::<u16>().unwrap(),
            db: env_var("REDIS_DB").parse::<u8>().unwrap(),
        }
    }

    pub fn url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}

impl Default for RedisSettings {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6379,
            db: 0,
        }
    }
}
