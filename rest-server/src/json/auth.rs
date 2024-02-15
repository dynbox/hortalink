use garde::Validate;
use oauth2::CsrfToken;
use serde::{Deserialize, Serialize};
use crate::json::validations::validate_name;

#[derive(Debug, Clone, Deserialize)]
pub enum Credentials {
    Password(LoginCreds),
    OAuth(OAuthCreds),
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCreds {
    pub user: UserInfo,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginCreds {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct SignCreds {
    #[garde(email)]
    pub email: String,
    #[garde(custom(validate_name), length(min = 8))]
    #[garde(required)]
    pub name: Option<String>,
    //#[garde(phone_number)]
    #[garde(skip)]
    pub phone: Option<String>,
    #[garde(length(min = 8))]
    pub password: String,
    #[garde(alphanumeric, length(min=3, max=16))]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthzResp {
    pub code: String,
    pub state: CsrfToken,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserInfo {
    pub email: String,
    pub name: String
}

#[derive(Serialize)]
pub struct AuthUrlResponse {
    pub auth_url: String,
}