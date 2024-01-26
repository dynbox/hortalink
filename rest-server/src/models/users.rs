use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    id: Uuid,
    pub email: String,
    pub password: Option<String>,
    access_token: Option<String>,
    avatar: Option<String>,
    username: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    address: Option<String>,
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
            self.access_token
                .as_ref()
                .unwrap()
                .as_bytes()
        }
    }
}