use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateNotificationPayload {
    pub read: bool
}