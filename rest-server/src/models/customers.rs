use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct CustomerUser {
    pub address: String
}