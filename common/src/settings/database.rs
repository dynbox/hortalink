use std::env::var;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub database: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl DatabaseSettings {
    pub fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            username: var("DB_USERNAME")
                .unwrap_or("postgres".to_string()),
            database: var("DB_NAME")
                .unwrap_or("hortalink".to_string()),
            password: var("DB_PASSWORD")
                .unwrap_or("postgres".to_string()),
            host: var("DB_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("DB_PORT")
                .unwrap_or("5432".to_string())
                .parse().unwrap(),
        }
    }
}