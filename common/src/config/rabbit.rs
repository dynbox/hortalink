use serde::{Deserialize, Serialize};

use super::env_var;

#[derive(Serialize, Deserialize)]
pub struct RabbitSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl RabbitSettings {
    pub(super) fn new_from_env() -> Self {
        Self {
            username: env_var("RABBIT_USERNAME"),
            password: env_var("RABBIT_PASSWORD"),
            host: env_var("RABBIT_HOST"),
            port: env_var("RABBIT_PORT").parse::<u16>().unwrap(),
        }
    }

    pub fn url(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

impl Default for RabbitSettings {
    fn default() -> Self {
        Self {
            username: String::default(),
            password: String::default(),
            host: "localhost".to_string(),
            port: 5672,
        }
    }
}
