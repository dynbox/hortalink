use crate::socket::server::SocketServer;

pub mod socket;
mod handshake;
pub mod json;
mod request;
mod events;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server = SocketServer::new()
        .await;

    server.run()
        .await
}