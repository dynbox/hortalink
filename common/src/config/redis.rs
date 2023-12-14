use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub db: u8,
}

impl RedisSettings {
    pub fn new() -> Self {
        Self { 
            host: env::var("REDIS_HOST").unwrap(), 
            port: env::var("REDIS_PORT").unwrap().parse::<u16>().unwrap(), 
            db: env::var("REDIS_DB").unwrap().parse::<u8>().unwrap(),
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
