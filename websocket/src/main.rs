use crate::app::Server;

mod emitters;
mod app;
mod handlers;
mod json;

#[tokio::main]
async fn main() {
    env_logger::init();
    let server = Server::new().await;
    server.run().await;
}