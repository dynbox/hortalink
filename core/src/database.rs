use log::{info, trace};
use sqlx::{Pool, Postgres};
use sqlx::pool::PoolOptions;
use sqlx::postgres::PgPoolOptions;
use common::config::database::DatabaseSettings;

pub struct SqlxManager {
    pub pool: Pool<Postgres>,
}

pub trait Table {
    fn statement() -> String;
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

    pub async fn register_tables(&self, statements: Vec<String>) {
        trace!("Trying to register {} tables", statements.len());

        for table in statements {
            sqlx::query(&table)
                .execute(&self.pool)
                .await
                .unwrap();
        }
    }

    fn configure_pool() -> PoolOptions<Postgres> {
        PgPoolOptions::new()
            .test_before_acquire(true)
            .max_connections(5)
    }
}