use std::io::{Error, ErrorKind};
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::Mutex;
use web_socket::WebSocket;

use crate::handlers::{Buff, Context};
use crate::handlers::identify::IdentifyEvent;

pub trait WebSocketEvent<Event> {
    async fn handle(
        ws: &WebSocket<Buff>,
        payload: Event
    ) -> Result<(), Error>;
}

pub async fn dispatch_event(
    ws: &WebSocket<Buff>, 
    buf: &[u8],
) -> Result<(), Error> {
    let message = String::from_utf8_lossy(buf);

    handle_event(ws, serde_json::from_str(&message)?)
        .await
}

async fn handle_event(
    ws: &WebSocket<Buff>, 
    message: Value,
) -> Result<(), Error> {
    let data = serde_json::from_value(message["d"].clone()).unwrap();
    
    match message["op"].as_i64().unwrap() {
        1 => IdentifyEvent::handle(ws, data)
            .await,
        _ => Err(Error::new(ErrorKind::NotFound, "Evento não registrado"))
    }
}