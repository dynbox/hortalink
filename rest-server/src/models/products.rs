use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Decimal;

use crate::json::serialize_timestamp;

#[derive(sqlx::FromRow, Serialize)]
pub struct SellerProduct {
    id: i32,
    #[sqlx(flatten)]
    product: Product,
    photos: Vec<String>,
    quantity: Option<i16>,
    price: Decimal,
    rating: Option<f64>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Product {
    #[sqlx(rename = "product_id")]
    id: i32,
    name: String,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct ProductComment {
    id: i32,
    customer_id: i32,
    #[serde(serialize_with = "serialize_timestamp")]
    created_at: NaiveDateTime,
    was_edited: Option<bool>,
    rating: i16,
    content: Option<String>,
}