use crate::app::web::WebApp;

mod routes;
mod models;
pub mod json;
mod app;

#[tokio::main]
async fn main() {
    env_logger::init();

    WebApp::new().await
        .serve()
        .await
}
