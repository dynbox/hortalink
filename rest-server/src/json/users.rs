use garde::Validate;
use serde::{Deserialize, Serialize};
use crate::json::validations::validate_name;

#[derive(Deserialize, Validate)]
pub struct UpdateUser {
    #[garde(skip)]
    pub avatar: Option<String>,
    #[garde(alphanumeric, length(min=3, max=16))]
    pub username: Option<String>,
    #[garde(custom(validate_name), length(min = 8))]
    pub name: Option<String>,
    //#[garde(phone_number)]
    #[garde(skip)]
    pub phone: Option<String>,
    #[garde(skip)]
    pub address: Option<String>,
    #[garde(email)]
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct UserMe {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    pub avatar: Option<String>,
    pub account_type: i32,
    pub bio: Option<String>
}