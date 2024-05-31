use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;

use common::entities::{StarRating, WeekDay};

use crate::json::validate_price;
use crate::models::products::SellerProduct;
use crate::models::schedules::Schedule;

#[derive(Serialize)]
pub struct SellerProductResponse {
    pub product: SellerProduct,
    pub schedule: Vec<Schedule>,
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
    #[garde(skip)]
    pub schedule_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PatchSellerProduct {
    #[garde(custom(validate_price))]
    pub price: Option<Decimal>,
    #[garde(range(min = 1))]
    pub quantity: Option<i16>,
    #[garde(length(min = 1, max = 5))]
    pub photos: Option<Vec<String>>,
    #[garde(skip)]
    pub add_schedules: Option<Vec<i32>>,
    #[garde(skip)]
    pub remove_schedules: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct FilterProducts {
    #[garde(custom(validate_price))]
    pub max_price: Option<Decimal>,
    #[garde(custom(validate_price))]
    pub min_price: Option<Decimal>,
    #[garde(skip)]
    pub min_stars: Option<StarRating>,
    #[garde(range(min = 1, max = 100))]
    pub product_type: Option<i32>,
    #[garde(skip)]
    pub start_time: Option<time::Time>,
    #[garde(skip)]
    pub day_of_week: Option<WeekDay>,
    #[garde(range(min = 1, max = 100))]
    pub page: i16,
    #[garde(range(min = 5, max = 100))]
    pub per_page: i16,
    #[garde(range(min = - 90.0000000, max = 90.0000000))]
    pub latitude: Option<f64>,
    #[garde(range(min = - 180.0000000, max = 180.0000000))]
    pub longitude: Option<f64>,
}