use std::path::Path;
use log::{error, info};
use serde::{Deserialize, Serialize};
use crate::settings::database::DatabaseSettings;
use crate::settings::secrets::Secrets;
use crate::settings::web::WebApp;

pub mod web;
pub mod database;
pub mod secrets;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AppSettings {
    pub web: WebApp,
    pub database: DatabaseSettings,
    pub secrets: Secrets,
}

impl AppSettings {
    pub fn new(path: &str) -> Self {
        let path = Path::new(path);

        if path.exists() {
            info!("Config file found. Reading configurations from file: '{}'", path.display());

            return Self::read(path)
        }

        let environment = std::env::var("ENVIRONMENT")
            .unwrap_or("PRODUCTION".to_string());

        match environment.as_str() {
            "PRODUCTION" | "STAGE" => {
                std::env::set_var("RUST_LOG", "info error");
                info!("Reading configurations from '{}' or using default.", environment);

                // Read configs from env or use default
                Self::default()
            }
            "DEVELOPMENT" => {
                Self::write(path);
                error!(
                    "Default configurations written to '{}'. Please edit this file to continue.",
                    path.display()
                );

                std::process::exit(1)
            }
            _ => {
                error!("Invalid environment specified: {}. Exiting...", environment);

                std::process::exit(1)
            }
        }
    }

    fn read(path: &Path) -> Self {
        toml::from_str(&std::fs::read_to_string(path).unwrap())
            .expect("Error reading file")
    }

    fn write(path: &Path) {
        std::fs::write(path, toml::to_string(&Self::default()).unwrap())
            .expect("Error serializing to TOML");
    }
}