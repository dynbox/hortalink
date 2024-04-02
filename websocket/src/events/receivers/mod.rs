use sqlx::{Pool, Postgres};

use crate::json::error::SocketError;
use crate::json::event::EventData;
use crate::socket::session::SocketSession;

pub mod identify;
pub mod heartbeat;

pub trait ReceiverEvent {
    type Response;

    async fn execute(
        data: EventData,
        session: &mut SocketSession,
        pool: &Pool<Postgres>,
    ) -> Result<Self::Response, SocketError>;
}