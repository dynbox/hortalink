use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;

use crate::json::validate_price;
use crate::models::products::SellerProduct;
use crate::models::schedules::Schedule;

#[derive(Serialize)]
pub struct SellerProductResponse {
    pub product: SellerProduct,
    pub schedule: Option<Schedule>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PostSellerProduct {
    #[garde(range(min = 0))]
    pub product_id: i32,
    #[garde(required)]
    #[garde(custom(validate_price))]
    pub price: Option<Decimal>,
    #[garde(range(min = 1))]
    pub quantity: Option<i16>,
    #[garde(length(min = 1, max = 5))]
    pub photos: Vec<String>,
    #[garde(length(min = 1, max = 10))]
    pub schedules: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PatchSellerProduct {
    #[garde(custom(validate_price))]
    pub price: Option<Decimal>,
    #[garde(range(min = 1))]
    pub quantity: Option<i16>,
    #[garde(length(min = 1, max = 5))]
    pub photos: Option<Vec<String>>,
    #[garde(length(min = 1, max = 10))]
    pub remove_schedules: Option<Vec<i32>>,
    #[garde(length(min = 1, max = 10))]
    pub add_schedules: Option<Vec<i32>>,
}