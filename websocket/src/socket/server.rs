use std::collections::HashMap;

use sqlx::{Pool, Postgres};

use app_core::database::SqlxManager;
use common::settings::AppSettings;

pub struct SocketServer {
    pub settings: AppSettings,
    pub pool: Pool<Postgres>,
    pub connections: HashMap<usize, String>,
}

impl SocketServer {
    pub async fn new() -> Self {
        let settings = AppSettings::new("application.toml");
        let pool = SqlxManager::new(&settings.database).await.pool;

        Self {
            settings,
            pool,
            connections: HashMap::new(),
        }
    }
}