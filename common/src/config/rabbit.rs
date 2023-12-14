use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RabbitSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl RabbitSettings {
    pub fn new() -> Self {
        Self {
            username: env::var("RABBIT_USERNAME").unwrap(),
            password: env::var("RABBIT_PASSWORD").unwrap(),
            host: env::var("RABBIT_HOST").unwrap(),
            port: env::var("RABBIT_PORT").unwrap().parse::<u16>().unwrap(),
        }
    }

    pub fn url(&self) -> String {
        format!("amqp://{}:{}@{}:{}", self.username, self.password, self.host, self.port)
    }
}

impl Default for RabbitSettings {
    fn default() -> Self {
        Self {
            username: String::default(),
            password: String::default(),
            host: "localhost".to_string(),
            port: 5672,
        }
    }
}
