use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use password_auth::verify_password;
use sqlx::{Pool, Postgres};
use sqlx::types::Uuid;
use crate::json::auth::Credentials;
use crate::models::users::ProtectedUser;

#[derive(Clone)]
pub struct Backend {
    db: Pool<Postgres>,
}

impl Backend {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Sqlx(sqlx::Error),
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = ProtectedUser;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        match creds {
            Credentials::Password(creds) => {
                let user: Option<Self::User> = sqlx::query_as("SELECT * FROM USERS WHERE email = $1")
                    .bind(creds.email)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(Self::Error::Sqlx)?;

                Ok(user.filter(|user| {
                    let Some(user_password) = &user.password else {
                        return false;
                    };

                    verify_password(creds.password, user_password)
                        .ok()
                        .is_some()
                }))
            }
            Credentials::OAuth(creds) => {
                let user = sqlx::query_as(
                    r#"
                    INSERT INTO users (email, name, access_token) VALUES ($1, $2, $3)
                    ON CONFLICT(email) DO UPDATE
                    SET access_token = excluded.access_token
                    RETURNING *
                    "#,
                )
                    .bind(creds.user.email)
                    .bind(creds.user.name)
                    .bind(creds.token)
                    .fetch_one(&self.db)
                    .await
                    .map_err(Self::Error::Sqlx)?;

                Ok(Some(user))
            }
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(Uuid::parse_str(user_id).unwrap())
            .fetch_optional(&self.db)
            .await
            .map_err(Self::Error::Sqlx)?;

        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;