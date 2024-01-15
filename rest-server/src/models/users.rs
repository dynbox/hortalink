use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    id: Uuid,
    name: Option<String>,
    pub email: String,
    phone: Option<i32>,
    account_type: i32,
    pub password: Option<String>,
    address: Option<String>,
    avatar: Option<String>,
    bio: Option<String>
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        if let Some(password) = &self.password {
            password.as_bytes()
        } else {
            "".as_bytes()
        }
    }
}