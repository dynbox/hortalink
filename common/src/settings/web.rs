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
    pub proxy: String,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub ssl: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebClient {
    pub host: String,
    pub port: u16,
    pub proxy: String,
    #[serde(skip)]
    #[serde(default)]
    pub ssl: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CdnServer {
    pub host: String,
    pub port: u16,
    pub storage: String,
    pub proxy: String,
    #[serde(skip)]
    #[serde(default)]
    pub ssl: bool,
}

impl Protocol for RestServer {
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

impl Default for RestServer {
    fn default() -> Self {
        Self {
            host: var("REST_SERVER_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("REST_SERVER_PORT")
                .unwrap_or("5443".to_string())
                .parse().unwrap(),
            proxy: var("DEFAULT_PROXY")
                .unwrap_or("hortalink.dev".to_string())
                .parse().unwrap(),
            ssl: true,
        }
    }
}

impl Protocol for WebClient {
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

impl Default for WebClient {
    fn default() -> Self {
        Self {
            host: var("WEB_CLIENT_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("WEB_CLIENT_PORT")
                .unwrap_or("5173".to_string())
                .parse().unwrap(),
            proxy: var("DEFAULT_PROXY")
                .unwrap_or("hortalink.dev".to_string())
                .parse().unwrap(),
            ssl: true,
        }
    }
}

impl Protocol for CdnServer {
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
            proxy: var("CDN_PROXY")
                .unwrap_or("cdn.hortalink.dev".to_string())
                .parse().unwrap(),
            ssl: true,
        }
    }
}