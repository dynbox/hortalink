mod identify;
mod heartbeat;

use std::collections::HashMap;
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;
use web_socket::Event;
use crate::events::receivers::heartbeat::HeartbeatEvent;
use crate::events::receivers::identify::IdentifyEvent;
use crate::json::error::SocketError;
use crate::json::event::{EventData, SocketRequest};
use crate::socket::session::SocketSession;

pub trait ReceiverEvent {
    type Response;

    async fn execute(
        data: EventData,
        session: &mut SocketSession,
    ) -> Result<Self::Response, SocketError>;
}

pub async fn handle_event(
    session: &mut SocketSession,
    event: &Event
) -> Result<(), ()> {
    match event {
        Event::Data { data, .. } => {
            let data = serde_json::from_slice::<SocketRequest>(&data).unwrap();

            match data.op {
                0 => IdentifyEvent::execute(data.d.unwrap(), session),
                _ => return Err(()),
            }.await.expect("TODO: panic message");
        }
        Event::Pong(_) => HeartbeatEvent::execute(EventData::Heartbeat, session).await.unwrap(),
        Event::Error(_) | Event::Close { .. } => return Err(()),
        _ => {}
    };
    
    Ok(())
}