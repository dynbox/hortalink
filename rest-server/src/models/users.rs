use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow, Row};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    pub email: String,
    pub password: Option<String>,
    access_token: Option<String>,
    avatar: Option<String>,
    username: Option<String>,
    name: Option<String>,
    phone: Option<i32>,
    address: Option<String>,
}

impl<'r> FromRow<'r, sqlx::postgres::PgRow> for User {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get::<Uuid, _>("id")?.to_string(),
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            access_token: row.try_get("access_token")?,
            avatar: row.try_get("avatar")?,
            username: row.try_get("username")?,
            name: row.try_get("name")?,
            phone: row.try_get("phone")?,
            address: row.try_get("address")?,
        })
    }
}


impl AuthUser for User {
    type Id = String;

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