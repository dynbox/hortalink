use std::time::Instant;
use sqlx::{Pool, Postgres};
use crate::events::receivers::ReceiverEvent;
use crate::json::error::SocketError;
use crate::json::event::EventData;
use crate::socket::session::SocketSession;

pub struct HeartbeatEvent;

impl ReceiverEvent for HeartbeatEvent {
    type Response = ();

    async fn execute(_: EventData, session: &mut SocketSession, _: &Pool<Postgres>,) -> Result<Self::Response, SocketError> {
        session.hb = Instant::now();

        Ok(())
    }
}