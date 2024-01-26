use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub avatar: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}