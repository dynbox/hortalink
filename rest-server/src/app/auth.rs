use std::collections::HashSet;
use axum::async_trait;
use axum_login::{AuthnBackend, AuthzBackend, UserId};
use password_auth::verify_password;
use sqlx::{Pool, Postgres};
use common::entities::UserRole;
use crate::json::auth::Credentials;
use crate::models::users::LoginUser;

#[derive(Clone)]
pub struct AuthGate {
    db: Pool<Postgres>,
}

impl AuthGate {
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
impl AuthnBackend for AuthGate {
    type User = LoginUser;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        match creds {
            Credentials::Password(creds) => {
                let user = sqlx::query_as::<_, Self::User>(
                    r#"
                        SELECT
                            id, password,
                            permissions, access_token
                        FROM USERS
                        WHERE email = $1
                    "#
                )
                    .bind(creds.email)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(Self::Error::Sqlx)?;

                let user = user.filter(|user| {
                    let Some(user_password) = &user.password else {
                        return false;
                    };

                    verify_password(creds.password, user_password)
                        .ok()
                        .is_some()
                });

                Ok(user)
            }
            Credentials::OAuth(creds) => {
                let user = sqlx::query_as(
                    r#"
                        INSERT INTO users (email, name, access_token)
                        VALUES ($1, $2, $3)
                        ON CONFLICT(email) DO UPDATE
                        SET access_token = excluded.access_token
                        RETURNING
                            id, password,
                            permissions, access_token
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
        let user = sqlx::query_as(
            r#"
                SELECT
                    id, password,
                    permissions, access_token
                FROM users WHERE id = $1
            "#
        )
            .bind(user_id)
            .fetch_optional(&self.db)
            .await
            .map_err(Self::Error::Sqlx)?;

        Ok(user)
    }
}

#[async_trait]
impl AuthzBackend for AuthGate {
    type Permission = UserRole;

    async fn get_user_permissions(
        &self,
        user: &Self::User
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let permissions = HashSet::from_iter(vec![user.permissions.clone()]);

        Ok(permissions)
    }
}

pub type AuthSession = axum_login::AuthSession<AuthGate>;