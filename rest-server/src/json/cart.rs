use garde::Validate;
use serde::Deserialize;
use sqlx::types::chrono::NaiveTime;

#[derive(Deserialize, Validate)]
pub struct PatchProductCart {
    #[garde(skip)]
    pub withdrawn: Option<NaiveTime>,
    #[garde(range(min = 1, max = 20))]
    pub amount: Option<i32>
}

#[derive(Deserialize, Validate)]
pub struct PostProductCart {
    #[garde(skip)]
    pub seller_product_id: i32,
    #[garde(skip)]
    pub withdrawn: NaiveTime,
    #[garde(range(min = 1, max = 20))]
    pub amount: i32
}