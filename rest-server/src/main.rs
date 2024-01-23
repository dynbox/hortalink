use crate::app::WebApp;

mod routes;
mod models;
mod app;
pub mod json;
mod provider;

#[tokio::main]
async fn main() {
    env_logger::init();

    WebApp::new().await
        .serve()
        .await
}
