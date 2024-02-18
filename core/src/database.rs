use log::info;
use sqlx::{Pool, Postgres};
use sqlx::pool::PoolOptions;
use sqlx::postgres::PgPoolOptions;
use common::settings::database::DatabaseSettings;

pub struct SqlxManager {
    pub pool: Pool<Postgres>,
}

impl SqlxManager {
    pub async fn new(settings: &DatabaseSettings) -> Self {
        Self {
            pool: Self::connect(settings).await
        }
    }

    async fn connect(settings: &DatabaseSettings) -> Pool<Postgres> {
        let pool_options = Self::configure_pool();

        info!("Trying to connect to Postgres server...");

        pool_options.connect(&settings.url())
            .await
            .expect("Failed to connect to Postgres server")
    }

    fn configure_pool() -> PoolOptions<Postgres> {
        PgPoolOptions::new()
            .test_before_acquire(true)
            .max_connections(5)
    }
}