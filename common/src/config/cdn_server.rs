use serde::{Deserialize, Serialize};

use super::{database::DatabaseSettings, env_var};

#[derive(Serialize, Deserialize)]
pub struct CdnServer {
    pub host: String,
    pub port: u16,
    pub database: DatabaseSettings,
}

impl CdnServer {
    pub(super) fn new_from_env() -> Self {
        Self {
            host: env_var("CDN_SERVER_HOST"),
            port: env_var("CDN_SERVER_PORT").parse::<u16>().unwrap(),
            database: DatabaseSettings::new_from_env("cdn"),
        }
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub(super) fn echo_env(&self) {
        std::env::set_var("CDN_SERVER_URL", self.url())
    }
}

impl Default for CdnServer {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 80,
            database: DatabaseSettings::default(),
        }
    }
}