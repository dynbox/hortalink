use std::path::Path;

use axum::{Extension, Router};
use axum::http::{header, Method};
use image::ImageFormat;
use image::imageops::FilterType;
use tower_http::cors::CorsLayer;

use common::settings::{AppSettings, Protocol};

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

        Self::load_resources(&settings.web.cdn.storage);

        Self {
            state: AppState {
                settings,
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

    fn load_resources(storage_path: &String) {
        let storage_resources = format!("{storage_path}/resources");
        let path = Path::new(storage_resources.as_str());

        if path.exists() {
            return;
        } else {
            std::fs::create_dir(path).unwrap();
        }

        let resources = Path::new("cdn-server/resources");

        process_directory(resources, path);
    }
}

fn process_directory(path: &Path, destination: &Path) {
    if !path.exists() {
        return;
    }

    if !destination.exists() {
        std::fs::create_dir(destination);
    }

    log::info!("Loading images from {:?}", path);
    
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(file) = entry {
                if file.file_type().unwrap().is_dir() {
                    process_directory(file.path().as_path(), destination.join(file.file_name()).as_path());
                } else {
                    let image = image::open(file.path()).unwrap();
                    let image = image.resize(512, 512, FilterType::Gaussian);

                    let name = file.file_name();
                    let name = name.to_str().unwrap().split(".").collect::<Vec<&str>>();

                    image.save_with_format(
                        destination.join(name.first().unwrap()), 
                        ImageFormat::from_extension(name.last().unwrap()).unwrap()
                    ).unwrap();
                }
            }
        }
    };
}