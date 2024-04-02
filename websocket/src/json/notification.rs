use serde::{Deserialize, Serialize};
use common::entities::NotificationType;

#[derive(Serialize, Deserialize)]
pub struct UserNotification {
    id: i32,
    pub user_id: i32,
    title: String,
    content: String,
    read: bool,
    #[serde(rename = "type")]
    notification_type: NotificationType,
    icon: Option<String>
}