use serde::Serialize;
use sqlx::types::time::Time;
use common::entities::WeekDay;
use crate::json::serialize_time;

#[derive(sqlx::FromRow, Serialize)]
pub struct Schedule {
    id: i32,
    address: String,
    #[serde(serialize_with = "serialize_time")]
    start_time: Time,
    #[serde(serialize_with = "serialize_time")]
    end_time: Time,
    #[sqlx(try_from = "i16")]
    day_of_week: WeekDay,
}