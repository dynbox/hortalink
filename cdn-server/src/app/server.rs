use axum::{Extension, Router};
use axum::http::{header, Method};
use tower_http::cors::CorsLayer;
use app_core::database::SqlxManager;

use common::settings::AppSettings;
use common::settings::web::WebServer;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
}

pub struct Server {
    pub state: AppState,
}

impl Server {
    pub async fn new(settings: &str) -> Self {
        let settings = AppSettings::new(settings);

        Self {
            state: AppState {
                settings
            }
        }
    }
    
    pub fn router(state: AppState) -> Router {
        Router::new()
            .merge(routes::router())
            .layer(Self::configure_cors(&state))
            .layer(Extension(state))
    }

    pub async fn run(self) {
        log::info!("Starting axum server with tokio...");

        let listener = tokio::net::TcpListener::bind(self.state.settings.web.cdn.url())
            .await
            .unwrap();
        axum::serve(listener, Self::router(self.state))
            .await
            .expect("Failed to start axum server");
    }

    fn configure_cors(state: &AppState) -> CorsLayer {
        CorsLayer::new()
            .allow_credentials(true)
            .allow_origin([
                state.settings.web.client.protocol_url().parse()
                    .unwrap()
            ])
            .allow_headers([
                header::CONTENT_TYPE
            ])
            .allow_methods([
                Method::GET, Method::PUT,
                Method::DELETE, Method::PATCH,
            ])
    }
}