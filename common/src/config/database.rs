use serde::{Deserialize, Serialize};

use super::env_var;

#[derive(Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub database: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl DatabaseSettings {
    pub(super) fn new_from_env(database: &str) -> Self {
        let env_db = database.replace('-', "_").to_uppercase();

        Self {
            username: env_var(format!("DB_{env_db}_USERNAME")),
            database: database.to_string(),
            password: env_var(format!("DB_{env_db}_PASSWORD")),
            host: env_var(format!("DB_{env_db}_HOST")),
            port: env_var(format!("DB_{env_db}_PORT")).parse::<u16>().unwrap(),
        }
    }

    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            username: String::default(),
            database: String::default(),
            password: String::default(),
            host: "localhost".to_string(),
            port: 5432,
        }
    }
}
