use garde::Validate;
use serde::{Deserialize, Serialize};
use common::entities::UserRole;

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

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct LoginCreds {
    #[garde(email)]
    pub email: String,
    #[garde(length(min=8, max=64))]
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserInfo {
    pub email: String,
    pub name: String
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SignCreds {
    #[garde(skip)]
    pub name: String,
    #[garde(length(min=2, max=16))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min=8, max=64))]
    pub password: String,
    #[garde(skip)]
    pub avatar: Option<String>,
    #[garde(custom(validate_account_role))]
    pub role: UserRole
}

fn validate_account_role(value: &UserRole, _: &()) -> garde::Result {
    match value {
        UserRole::Customer |  UserRole::Seller => Ok(()),
        _ => Err(garde::Error::new("Invalid account role")),
    }
}
