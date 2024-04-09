use serde::Serialize;
use sqlx::{Pool, Postgres};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Decimal;

use crate::json::error::ApiError;

#[derive(sqlx::FromRow, Serialize)]
pub struct Order {
    #[sqlx(rename = "order_id")]
    id: i32,
    #[sqlx(flatten)]
    customer: UserPreview,
    amount: i32,
    withdrawn: NaiveDateTime,
    #[sqlx(flatten)]
    product: ProductPreview,
}

#[derive(sqlx::FromRow, Serialize)]
struct UserPreview {
    #[sqlx(rename = "user_id")]
    id: i32,
    name: String,
    avatar: Option<String>,
}

#[derive(sqlx::FromRow, Serialize)]
struct ProductPreview {
    #[sqlx(rename = "product_id")]
    id: i32,
    #[sqlx(rename = "product_name")]
    name: String,
    price: Decimal,
    photo: Option<String>,
}

impl Order {
    pub async fn get_seller(pool: &Pool<Postgres>, order_id: i32) -> Result<i32, ApiError> {
        let seller_id = sqlx::query_scalar::<_, i32>(
            r#"
                SELECT sp.seller_id
                FROM cart c
                JOIN "seller_products" sp ON sp.id = c.seller_product_id
                WHERE c.id = $1
            "#
        )
            .bind(order_id)
            .fetch_optional(pool)
            .await?;

        if seller_id.is_none() {
            return Err(ApiError::NotFound("Produto não encontrada".to_string()));
        }

        Ok(seller_id.unwrap())
    }

    pub async fn get_customer(pool: &Pool<Postgres>, order_id: i32) -> Result<i32, ApiError> {
        let customer_id = sqlx::query_scalar::<_, i32>(
            r#"
                SELECT customer_id
                FROM cart
                WHERE id = $1
            "#
        )
            .bind(order_id)
            .fetch_optional(pool)
            .await?;

        if customer_id.is_none() {
            return Err(ApiError::NotFound("Produto não encontrada".to_string()));
        }

        Ok(customer_id.unwrap())
    }
}