use std::env::var;
use serde::{Deserialize, Serialize};

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

pub trait WebServer {
    fn get_host(&self) -> &String;

    fn get_port(&self) -> &u16;

    fn url(&self) -> String {
        format!("{}:{}", self.get_host(), self.get_port())
    }

    fn protocol_url(&self) -> String {
        let protocol = if &self.get_host() == &"localhost" { "http" } else { "https" };

        format!("{}://{}", protocol, self.url())
    }
}

impl WebServer for RestServer {
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

impl WebServer for WebClient {
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

impl WebServer for CdnServer {
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