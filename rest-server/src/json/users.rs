use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub avatar: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone: Option<i32>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}