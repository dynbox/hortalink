use garde::Validate;
use serde::{Deserialize, Serialize};

use common::entities::StarRating;

#[derive(Serialize, Deserialize, Validate)]
pub struct PostSellerRating {
    #[garde(skip)]
    pub rating: StarRating,
    #[garde(length(min = 1, max = 5))]
    pub tags: Option<Vec<i16>>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PatchSellerRating {
    #[garde(skip)]
    pub rating: Option<StarRating>,
    #[garde(length(min = 1, max = 5))]
    pub tags: Option<Vec<i16>>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PostSellerProductRating {
    #[garde(skip)]
    pub rating: StarRating,
    #[garde(length(min = 5, max = 256))]
    pub content: Option<String>
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PatchSellerProductRating {
    #[garde(skip)]
    pub rating: Option<StarRating>,
    #[garde(length(min = 5, max = 256))]
    pub content: Option<String>
}