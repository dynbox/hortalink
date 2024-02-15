use axum::Router;
use common::config::Settings;

mod routes;
mod json;

#[tokio::main]
async fn main() {
    let settings = Settings::new("application.toml");

    let router = Router::new()
        .merge(
            routes::avatars::router()
        );

    let listener = tokio::net::TcpListener::bind(settings.cdn_server.url())
        .await
        .unwrap();
    axum::serve(listener, router)
        .await
        .expect("Failed to start axum server");
}