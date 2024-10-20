use serde::Serialize;
use sqlx::{Pool, Postgres};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Decimal;

use crate::json::error::ApiError;
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
    author_id: i32,
    #[serde(serialize_with = "serialize_timestamp")]
    created_at: NaiveDateTime,
    was_edited: Option<bool>,
    rating: i16,
    content: Option<String>,
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