use serde::Serializer;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::time::Time;

pub mod auth;
pub mod error;
pub mod users;
pub mod notifications;
pub mod schedules;
pub mod sellers;
pub mod ratings;

pub fn serialize_timestamp<S>(naive_datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let timestamp = naive_datetime.timestamp();
    serializer.serialize_i64(timestamp)
}

pub fn serialize_time<S>(time: &Time, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_str(&format!("{:02}:{:02}", time.hour(), time.minute()))
}