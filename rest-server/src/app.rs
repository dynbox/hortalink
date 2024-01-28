use axum::{Extension, Router};
use axum_login::{login_required, AuthManagerLayer, AuthManagerLayerBuilder};
use http::header::CONTENT_TYPE;
use http::{HeaderValue, Method};
use sqlx::{Pool, Postgres};
use tower_http::cors::Any;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions::cookie::time::Duration;
use tower_sessions_sqlx_store::PostgresStore;
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

    async fn router(&self) -> Router {
        let state = AppState {
            pool: self.database.pool.clone(),
            settings: self.settings.clone(),
        };

        let cors = tower_http::cors::CorsLayer::default()
            .allow_origin(state.settings.webapp.protocol_url().parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PATCH])
            .allow_headers([CONTENT_TYPE])
            .allow_credentials(true);

        Router::new()
            .nest("/api",
                  routes::auth::router()
                      .merge(
                        routes::users::router()
                            .route_layer(login_required!(Backend))
                    )
            )
            .with_state(state)
            .layer(self.auth_layer())
            .layer(Extension(AuthProviders::new(&self.settings)))
            .layer(cors)
    }

    fn session(&self) -> SessionManagerLayer<PostgresStore> {
        let session_store = PostgresStore::new(self.database.pool.clone())
            .with_table_name("sessions")
            .unwrap();

        SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_name("session_id")
            .with_expiry(Expiry::OnInactivity(Duration::days(10)))
            .with_domain("localhost".to_string())
            .with_secure(false)
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
        axum::serve(listener, self.router().await)
            .await
            .expect("Failed to start axum server");
    }
}