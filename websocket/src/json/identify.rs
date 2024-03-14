use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdentifyPayload {
    pub session_id: String
}