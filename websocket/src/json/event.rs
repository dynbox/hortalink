use serde::{Deserialize, Serialize};
use crate::json::notification::UserNotification;

#[derive(Serialize, Deserialize)]
pub struct SocketRequest {
    pub op: u32,
    pub d: Option<EventData>
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    #[serde(skip)]
    Heartbeat,
    Notification(UserNotification),
    Identify {
        session: String
    }
}