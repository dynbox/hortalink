use serde::{Deserialize, Serialize};

use super::{database::DatabaseSettings, env_var};

#[derive(Serialize, Deserialize, Clone)]
pub struct WebServerSettings {
    pub host: String,
    pub port: u16,
    pub database: DatabaseSettings,
}

impl WebServerSettings {
    pub(super) fn new_from_env() -> Self {
        Self {
            host: env_var("API_SERVER_HOST"),
            port: env_var("API_SERVER_PORT").parse::<u16>().unwrap(),
            database: DatabaseSettings::new_from_env("hortalink"),
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
            database: DatabaseSettings::default(),
        }
    }
}
