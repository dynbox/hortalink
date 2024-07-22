use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::json::notifications::NotificationCreated;

pub mod user;
pub mod notifications;

#[derive(Serialize, Deserialize)]
pub struct GatewayRequest {
    pub opcode: u32,
    pub d: Option<GatewayData>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum GatewayData {
    // opcode: 10
    Identify { session_id: String },
    // opcode: 11
    NotificationCreated(NotificationCreated),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostgresEvent {
    NotificationCreated(NotificationCreated)
}

pub enum Command {
    Disconnect(SocketAddr),
    Identify(SocketAddr, String),
    NotificationCreated(NotificationCreated),
}

impl GatewayRequest {
    pub fn new(opcode: u32, d: Option<GatewayData>) -> Self {
        Self { opcode, d }
    }
}