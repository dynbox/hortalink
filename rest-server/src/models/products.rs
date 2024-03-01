use serde::Serialize;
use sqlx::types::Decimal;

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