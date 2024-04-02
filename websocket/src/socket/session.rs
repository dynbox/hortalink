use std::net::SocketAddr;
use std::time::Instant;

use tokio::sync::mpsc;

use crate::json::event::SocketRequest;

#[derive(Clone)]
pub struct SocketSession {
    pub hb: Instant,
    pub addr: SocketAddr,
    pub frame: mpsc::UnboundedSender<SocketRequest>,
}

impl SocketSession {
    pub fn new(addr: SocketAddr, frame: mpsc::UnboundedSender<SocketRequest>) -> Self {
        Self {
            hb: Instant::now(),
            addr,
            frame,
        }
    }
}