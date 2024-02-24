use serde::Serialize;
use sqlx::types::time::Time;
use crate::json::serialize_time;

#[derive(sqlx::FromRow, Serialize)]
pub struct Schedule {
    id: i32,
    address: String,
    #[serde(serialize_with = "serialize_time")]
    start_time: Time,
    #[serde(serialize_with = "serialize_time")]
    end_time: Time,
    day_of_week: i16,
}