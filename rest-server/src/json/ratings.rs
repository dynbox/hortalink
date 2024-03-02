use garde::Validate;
use serde::{Deserialize, Serialize};

use common::entities::StarRating;

#[derive(Serialize, Deserialize, Validate)]
pub struct PostRatingPayload {
    #[garde(range(min = 1, max = 5))]
    pub rating: i16,
    #[garde(length(min = 1, max = 5))]
    pub tags: Option<Vec<StarRating>>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PatchRatingPayload {
    #[garde(range(min = 1, max = 5))]
    pub rating: Option<i16>,
    #[garde(length(min = 1, max = 5))]
    pub tags: Option<Vec<StarRating>>,
}