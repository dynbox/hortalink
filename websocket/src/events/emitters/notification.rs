use std::collections::HashMap;
use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use crate::events::emitters::EmitterEvent;
use crate::json::error::SocketError;
use crate::socket::session::SocketSession;

pub struct NotificationEvent;

impl EmitterEvent for NotificationEvent {
    type Response = ();

    async fn execute(
        connections: &Arc<Mutex<HashMap<i32, SocketSession>>>,
        pool: &Pool<Postgres>,
        data: String,
    ) -> Result<Self::Response, SocketError> {
        println!("{data} : {}", connections.lock().await.len());

        Ok(())
    }
}