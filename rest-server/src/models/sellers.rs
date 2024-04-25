use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct SellerUser {
    pub bio: Option<String>
}