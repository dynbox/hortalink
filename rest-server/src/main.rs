use axum::Router;
use core::database::SqlxManager;
use common::config::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::new("application.toml");
    let database = SqlxManager::new(&settings.webserver.database)
        .await;

    let router = Router::new()
        .with_state(database.pool);

    let listener = tokio::net::TcpListener::bind(settings.webserver.url())
        .await
        .unwrap();
    axum::serve(listener, router)
        .await
        .expect("Failed to start axum server");
}
