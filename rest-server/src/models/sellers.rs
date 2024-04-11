use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct SellerUser {
    pub bio: Option<String>
}

#[derive(sqlx::FromRow, Serialize)]
pub struct PublicSeller {
    id: i32,
    name: String,
    avatar: Option<String>,
    bio: Option<String>,
    //rating: Option<f64>
}