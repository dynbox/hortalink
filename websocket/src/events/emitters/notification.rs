use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::events::emitters::EmitterEvent;
use crate::json::error::SocketError;
use crate::json::event::{EventData, SocketRequest};
use crate::json::notification::UserNotification;
use crate::socket::session::SocketSession;

pub struct NotificationEvent;

impl EmitterEvent for NotificationEvent {
    type Response = ();
    type Data = UserNotification;

    async fn execute(
        connections: &Arc<Mutex<HashMap<i32, SocketSession>>>,
        data: Self::Data,
    ) -> Result<Self::Response, SocketError> {
        let con = connections.lock().await;

        if let Some(user) = con.get(&data.user_id) {
            user.frame.send(SocketRequest {
                op: 17,
                d: Some(EventData::Notification(data)),
            }).expect("TODO: panic message");
        };

        Ok(())
    }
}