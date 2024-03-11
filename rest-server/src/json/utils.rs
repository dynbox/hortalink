use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate)]
pub struct Pagination {
    #[garde(range(min = 1, max = 100))]
    pub page: i16,
    #[garde(range(min = 5, max = 100))]
    pub per_page: i16
}