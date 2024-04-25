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