use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub database: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl DatabaseSettings {
    pub fn new(database: &str) -> Self {
        let env_db = database.replace('-', "_").to_uppercase();

        Self {
            username: env::var(format!("DB_{env_db}_USERNAME")).unwrap(),
            database: database.to_string(),
            password: env::var(format!("DB_{env_db}_PASSWORD")).unwrap(),
            host: env::var(format!("DB_{env_db}_HOST")).unwrap(),
            port: env::var(format!("DB_{env_db}_PORT")).unwrap().parse::<u16>().unwrap(),
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
