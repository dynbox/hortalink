use std::collections::HashSet;

use axum::async_trait;
use axum_login::{AuthnBackend, AuthzBackend, UserId};
use sqlx::{Pool, Postgres};

use crate::json::error::ApiError;
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

#[async_trait]
impl AuthnBackend for AuthGate {
    type User = LoginUser;
    type Credentials = ();
    type Error = ApiError;

    async fn authenticate(&self, _: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        Ok(None)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as(
            r#"
                SELECT
                    id, password,
                    roles, access_token
                FROM users WHERE id = $1
            "#
        )
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

#[async_trait]
impl AuthzBackend for AuthGate {
    type Permission = i16;

    async fn get_user_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let permissions = HashSet::from_iter(user.roles.clone());

        Ok(permissions)
    }
}

pub type AuthSession = axum_login::AuthSession<AuthGate>;