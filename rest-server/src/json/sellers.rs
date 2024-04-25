use serde::Serialize;

use crate::models::products::SellerProduct;
use crate::models::sellers::PublicSeller;

#[derive(Serialize)]
pub struct SellerResponse {
    pub seller: PublicSeller,
    pub products: Vec<SellerProduct>,
}