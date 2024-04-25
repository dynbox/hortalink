use serde::Serialize;
use crate::models::products::SellerProduct;
use crate::models::sellers::SellerUser;

#[derive(Serialize)]
pub struct SellerResponse {
    pub seller: SellerUser,
    pub products: Vec<SellerProduct>
}