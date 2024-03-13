use crate::app::Server;

mod emitters;
mod app;
mod handlers;

#[tokio::main]
async fn main() {
    env_logger::init();
    let server = Server::new();
    server.run().await;
}