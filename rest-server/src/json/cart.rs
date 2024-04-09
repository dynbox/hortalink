use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Validate)]
pub struct PatchProductCart {
    #[garde(skip)]
    pub withdrawn: Option<NaiveDateTime>,
    #[garde(range(min = 1, max = 20))]
    pub amount: Option<i32>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PostProductCart {
    #[garde(skip)]
    pub seller_product_id: i32,
    #[garde(skip)]
    pub withdrawn: NaiveDateTime,
    #[garde(range(min = 1, max = 20))]
    pub amount: i32,
}