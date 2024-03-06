use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;

use crate::json::serialize_timestamp;
use crate::models::users::PreviewUser;

#[derive(Serialize, sqlx::FromRow)]
pub struct ProductRatingInfo {
    id: i32,
    #[serde(serialize_with = "serialize_timestamp")]
    created_at: NaiveDateTime,
    was_edited: bool,
    rating: i16,
    content: String,
    #[sqlx(flatten)]
    user: PreviewUser
}