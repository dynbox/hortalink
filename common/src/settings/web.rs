use std::env::var;
use serde::{Deserialize, Serialize};
use crate::settings::Protocol;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct WebApp {
    pub rest: RestServer,
    pub client: WebClient,
    pub cdn: CdnServer,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RestServer {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebClient {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CdnServer {
    pub host: String,
    pub port: u16,
    pub storage: String
}

impl Protocol for RestServer {
    fn get_host(&self) -> &String {
        &self.host
    }

    fn get_port(&self) -> &u16 {
        &self.port
    }
}

impl Default for RestServer {
    fn default() -> Self {
        Self {
            host: var("REST_SERVER_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("REST_SERVER_PORT")
                .unwrap_or("5443".to_string())
                .parse().unwrap(),
        }
    }
}

impl Protocol for WebClient {
    fn get_host(&self) -> &String {
        &self.host
    }

    fn get_port(&self) -> &u16 {
        &self.port
    }
}

impl Default for WebClient {
    fn default() -> Self {
        Self {
            host: var("WEB_CLIENT_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("WEB_CLIENT_PORT")
                .unwrap_or("5173".to_string())
                .parse().unwrap(),
        }
    }
}

impl Protocol for CdnServer {
    fn get_host(&self) -> &String {
        &self.host
    }

    fn get_port(&self) -> &u16 {
        &self.port
    }
}

impl Default for CdnServer {
    fn default() -> Self {
        Self {
            host: var("CDN_SERVER_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("CDN_SERVER_PORT")
                .unwrap_or("5767".to_string())
                .parse().unwrap(),
            storage: var("CDN_STORAGE_PATH")
                .unwrap_or("/storage".to_string())
                .parse().unwrap(),
        }
    }
}