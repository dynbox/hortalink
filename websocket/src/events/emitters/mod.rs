use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::json::error::SocketError;
use crate::socket::session::SocketSession;

pub mod notification;

pub trait EmitterEvent {
    type Response;
    type Data;

    async fn execute(
        connections: &Arc<Mutex<HashMap<i32, SocketSession>>>,
        data: Self::Data,
    ) -> Result<Self::Response, SocketError>;
}