use oauth2::CsrfToken;
use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize)]
pub struct LoginCreds {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignCreds {
    pub email: String,
    pub name: String,
    pub phone: String,
    pub password: String,
    pub next: String,
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
