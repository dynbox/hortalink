use serde::Serializer;
use sqlx::types::chrono::NaiveDateTime;

pub mod auth;
pub mod error;
pub mod users;
pub mod notifications;

pub fn serialize_timestamp<S>(naive_datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let timestamp = naive_datetime.timestamp();
    serializer.serialize_i64(timestamp)
}