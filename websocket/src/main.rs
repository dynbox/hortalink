use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

use crate::socket::server::SocketServer;

mod socket;
mod handshake;

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = SocketServer::new()
        .await;
}