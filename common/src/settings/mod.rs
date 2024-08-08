use std::path::Path;
use std::str::FromStr;

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::entities::Environment;
use crate::settings::database::DatabaseSettings;
use crate::settings::secrets::Secrets;
use crate::settings::services::{RabbitMQ, WebSocket};
use crate::settings::web::WebApp;

pub mod web;
pub mod database;
pub mod secrets;
pub mod services;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AppSettings {
    pub web: WebApp,
    pub database: DatabaseSettings,
    pub rabbitmq: RabbitMQ,
    pub websocket: WebSocket,
    pub secrets: Secrets,
    #[serde(skip)]
    pub environment: Environment,
}

pub trait Protocol {
    fn get_host(&self) -> &String;

    fn get_port(&self) -> u16;

    fn get_proxy(&self) -> &String;

    fn is_ssl(&self) -> bool;

    fn socket(&self) -> String {
        format!("{}:{}", self.get_host(), self.get_port())
    }

    fn protocol_url(&self) -> String {
        let protocol = match (self.get_port(), self.is_ssl()) {
            (9002, true) => "wss",
            (9002, false) => "ws",
            (_, true) => "https",
            _ => "http",
        };

        format!("{}://{}", protocol, self.socket())
    }

    fn ssl_url(&self) -> String {
        let protocol = match (self.get_port(), self.is_ssl()) {
            (9002, true) => "wss",
            (9002, false) => "ws",
            (_, true) => "https",
            _ => "http",
        };

        format!("{}://{}", protocol, self.get_proxy())
    }
}

impl AppSettings {
    pub fn new(path: &str) -> Self {
        let path = Path::new(path);
        let environment = std::env::var("ENVIRONMENT")
            .map(|env| Environment::from_str(&env).unwrap())
            .unwrap_or(Environment::Production);

        if path.exists() {
            info!("Config file found. Reading configurations from file: '{}'", path.display());

            return Self::read(path, environment);
        }

        match environment {
            Environment::Production | Environment::Stage => {
                std::env::set_var("RUST_LOG", "info error");
                info!("Reading configurations from '{:?}' or using default.", environment);

                // Read configs from env or use default
                Self::default()
            }
            Environment::Development => {
                Self::write(path);
                error!(
                    "Default configurations written to '{}'. Please edit this file to continue.",
                    path.display()
                );

                std::process::exit(1)
            }
        }
    }

    fn read(path: &Path, environment: Environment) -> Self {
        let AppSettings {
            web,
            database,
            rabbitmq,
            websocket,
            secrets,
            ..
        } = toml::from_str(&std::fs::read_to_string(path).unwrap()).expect("Error reading file");
        
        Self {
            web,
            database,
            rabbitmq,
            websocket,
            secrets,
            environment,
        }
    }

    fn write(path: &Path) {
        std::fs::write(path, toml::to_string(&Self::default()).unwrap())
            .expect("Error serializing to TOML");
    }
}