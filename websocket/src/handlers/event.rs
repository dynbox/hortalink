use std::sync::Arc;
use sqlx::{Pool, Postgres};
use web_socket::WebSocket;
use crate::handlers::Buff;

trait WebSocketEvent<Event> {
    async fn handle(&self, ws: Arc<WebSocket<Buff>>, pool: &Pool<Postgres>, payload: Event);
}

pub async fn handle_event(ws: &WebSocket<Buff>, buf: &[u8]) {
    let _ = String::from_utf8_lossy(buf);
}