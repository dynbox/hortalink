use axum::{Extension, Router};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use sqlx::{Pool, Postgres};
use tower_sessions::{Expiry, PostgresStore, SessionManagerLayer};
use tower_sessions::cookie::time::Duration;
use app_core::database::SqlxManager;
use common::config::Settings;
use crate::provider::AuthProviders;
use crate::routes;
use crate::routes::auth::backend::Backend;

pub struct WebApp {
    settings: Settings,
    database: SqlxManager
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub settings: Settings,
}

impl WebApp {
    pub async fn new() -> Self {
        let settings = Settings::new("application.toml");
        let database = SqlxManager::new(&settings.webserver.database)
            .await;

        sqlx::migrate!().run(&database.pool).await.expect("Failed to register tables");

        Self {
            settings,
            database
        }
    }

    fn router(&self) -> Router {
        let state = AppState {
            pool: self.database.pool.clone(),
            settings: self.settings.clone(),
        };

        Router::new()
            .nest("/api",
                  routes::auth::router()
            )
            .with_state(state)
            .layer(self.auth_layer())
            .layer(Extension(AuthProviders::new(&self.settings)))
    }

    fn session(&self) -> SessionManagerLayer<PostgresStore> {
        let session_store = PostgresStore::new(self.database.pool.clone())
            .with_table_name("sessions")
            .unwrap();

        SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_name("session_id")
            .with_expiry(Expiry::OnInactivity(Duration::days(10)))
    }

    fn auth_layer(&self) -> AuthManagerLayer<Backend, PostgresStore> {
        let backend = Backend::new(
            self.database.pool.clone()
        );

        AuthManagerLayerBuilder::new(backend, self.session()).build()
    }

    pub async fn serve(&self) {
        let listener = tokio::net::TcpListener::bind(self.settings.webserver.url())
            .await
            .unwrap();
        axum::serve(listener, self.router())
            .await
            .expect("Failed to start axum server");
    }
}