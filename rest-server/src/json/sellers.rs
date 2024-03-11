use serde::Serialize;
use crate::models::products::SellerProduct;
use crate::models::sellers::PublicSeller;

#[derive(Serialize)]
pub struct SellerResponse {
    pub user: PublicSeller,
    pub products: Vec<SellerProduct>
}