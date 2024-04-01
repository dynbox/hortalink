use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum SocketError {
    #[error("Database")]
    #[serde(rename = "message")]
    Custom(String)
}