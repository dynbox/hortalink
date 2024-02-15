use axum_login::AuthUser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct PublicUser {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub avatar: String,
    pub account_type: i32,
    pub bio: String
}

#[derive(sqlx::FromRow)]
pub struct PublicMe {
    pub name: String,
    pub username: String,
    pub avatar: Option<String>,
    pub bio: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProtectedUser {
    pub id: i32,
    pub email: String,
    pub password: Option<String>,
    access_token: Option<String>,
    pub account_type: i32,
    pub permissions: i32
}

impl AuthUser for ProtectedUser {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        if let Some(password) = &self.password {
            password.as_bytes()
        } else {
            self.access_token
                .as_ref()
                .unwrap()
                .as_bytes()
        }
    }
}