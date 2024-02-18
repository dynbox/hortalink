use axum::http::{header, Method};
use axum::Router;
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use app_core::database::SqlxManager;
use common::settings::AppSettings;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub pool: Pool<Postgres>
}

pub struct Server {
    pub state: AppState,
}

impl Server {
    pub async fn new(settings: &str) -> Self {
        let settings = AppSettings::new(settings);
        let database = SqlxManager::new(&settings.database)
            .await;

        Self {
            state: AppState {
                settings,
                pool: database.pool
            }
        }
    }

    pub fn router(state: AppState) -> Router {
        let cors = CorsLayer::new()
            .allow_credentials(true)
            .allow_origin([
                state.settings.web.client.protocol_url().parse()
                    .unwrap()
            ])
            .allow_headers([
                header::AUTHORIZATION, header::CONTENT_TYPE,
            ])
            .allow_methods([
                Method::GET, Method::PUT,
                Method::DELETE, Method::PATCH,
            ]);

        Router::new()
            .nest("/api", crate::api::router())
            .layer(cors)
    }

    pub async fn run(self) {
        log::info!("Starting axum server with tokio...");

        let listener = tokio::net::TcpListener::bind(self.state.settings.web.rest.url())
            .await
            .unwrap();
        axum::serve(listener, Self::router(self.state))
            .await
            .expect("Failed to start axum server");
    }
}