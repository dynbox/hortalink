use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct SellerUser {
    pub bio: Option<String>
}

#[derive(sqlx::FromRow, Serialize)]
pub struct PublicSeller {
    pub name: String,
    pub id: i32,
    pub bio: Option<String>,
    pub avatar: Option<String>
}

#[derive(sqlx::FromRow, Serialize)]
pub struct PublicProfile {
    pub name: String,
    pub id: i32,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub is_seller: bool,
    pub followers: Option<i32>
}