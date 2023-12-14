use std::env;

use serde::{Deserialize, Serialize};

use super::database::DatabaseSettings;

#[derive(Serialize, Deserialize)]
pub struct WebServerSettings {
    pub host: String,
    pub port: u16,
    pub database: DatabaseSettings,
}

impl WebServerSettings {
    pub fn new() -> Self {
        Self {
            host: env::var("API_SERVER_HOST").unwrap(),
            port: env::var("API_SERVER_PORT").unwrap().parse::<u16>().unwrap(),
            database: DatabaseSettings::new("hortalink"),
        }
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub(super) fn echo_env(&self) {
        std::env::set_var("API_SERVER_URL", self.url())
    }
}

impl Default for WebServerSettings {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 443,
            database: DatabaseSettings::default()
        }
    }
}
