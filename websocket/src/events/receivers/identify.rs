use crate::events::receivers::ReceiverEvent;
use crate::json::error::SocketError;
use crate::json::event::EventData;
use crate::socket::session::SocketSession;

pub struct IdentifyEvent;

impl ReceiverEvent for IdentifyEvent {
    type Response = i32;

    async fn execute(data: EventData, session: &mut SocketSession) -> Result<Self::Response, SocketError> {
        todo!()
    }
}