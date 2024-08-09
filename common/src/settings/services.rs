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

#[derive(Serialize, Deserialize, Clone)]
pub struct WebSocket {
    pub host: String,
    pub port: u16,
    pub proxy: String,
    #[serde(skip)]
    #[serde(default)]
    pub ssl: bool
}

impl Protocol for RabbitMQ {
    fn get_host(&self) -> &String {
        &self.host
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn is_ssl(&self) -> bool {
        false
    }
}

impl Protocol for WebSocket {
    fn get_host(&self) -> &String {
        &self.host
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn get_proxy(&self) -> String {
        self.proxy.clone()
    }

    fn is_ssl(&self) -> bool {
        self.ssl
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
            username: var("RABBITMQ_USER")
                .unwrap_or("rabbit".to_string()),
            password: var("RABBITMQ_PASSWORD")
                .unwrap_or(String::new()),
        }
    }
}

impl Default for WebSocket {
    fn default() -> Self {
        Self {
            host: var("WEBSOCKET_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("WEBSOCKET_PORT")
                .unwrap_or("9002".to_string())
                .parse().unwrap(),
            proxy: var("WEBSOCKET_PROXY")
                .unwrap_or("gateway.hortalink.dev".to_string())
                .parse().unwrap(),
            ssl: true,
        }
    }
}