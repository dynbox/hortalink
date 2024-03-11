use std::env::var;

use serde::{Deserialize, Serialize};

use crate::settings::Protocol;

#[derive(Serialize, Deserialize, Clone)]
pub struct RabbitMQ {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Protocol for RabbitMQ {
    fn get_host(&self) -> &String {
        &self.host
    }

    fn get_port(&self) -> &u16 {
        &self.port
    }
}

impl Default for RabbitMQ {
    fn default() -> Self {
        Self {
            host: var("RABBITMQ_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("RABBITMQ_PORT")
                .unwrap_or("5672".to_string())
                .parse().unwrap(),
            username: var("RABBITMQ_USER").unwrap(),
            password: var("RABBITMQ_PASSWORD").unwrap(),
        }
    }
}