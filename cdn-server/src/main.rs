use crate::app::server::Server;

mod app;
pub mod routes;
pub mod json;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Server::new()
        .await;

    app.run().await;
}
