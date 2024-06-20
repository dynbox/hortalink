use serde::Serialize;
use sqlx::{Pool, Postgres};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Decimal;

use crate::json::error::ApiError;
use crate::json::serialize_timestamp;

#[derive(sqlx::FromRow, Serialize)]
pub struct SellerProduct {
    id: i64,
    #[sqlx(flatten)]
    product: Product,
    photos: Vec<String>,
    quantity: Option<i16>,
    price: Decimal,
    rating: Option<f64>,
    rating_quantity: Option<i32>,
    unit: Option<i32>
}

#[derive(sqlx::FromRow, Serialize)]
pub struct SellerProductPreview {
    id: i64,
    #[sqlx(flatten)]
    product: Product,
    price: Decimal,
    rating: Option<f64>,
    rating_quantity: Option<i32>,
    photos: Vec<String>,
    #[sqlx(skip)]
    dist: Option<f64>
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Product {
    #[sqlx(rename = "product_id")]
    id: i32,
    name: String,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct ProductComment {
    id: i64,
    author_id: i32,
    #[serde(serialize_with = "serialize_timestamp")]
    created_at: NaiveDateTime,
    was_edited: Option<bool>,
    rating: i16,
    content: Option<String>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct ResourceProduct {
    product_id: i32,
    product_name: String,
    alias: Vec<String>,
    category_name: String,
    category_id: i32
}

impl SellerProduct {
    pub async fn get_author(
        pool: &Pool<Postgres>,
        product_id: i32,
    ) -> Result<i32, ApiError> {
        let user_id = sqlx::query_scalar::<_, i32>(
            r#"
                SELECT seller_id FROM seller_products
                WHERE id = $1
            "#
        )
            .bind(product_id)
            .fetch_optional(pool)
            .await?;

        if user_id.is_none() {
            return Err(ApiError::NotFound("Produto não encontrada".to_string()));
        }

        Ok(user_id.unwrap())
    }
}