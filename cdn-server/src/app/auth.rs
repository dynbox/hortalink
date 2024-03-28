use std::collections::HashSet;

use axum::async_trait;
use axum_login::{AuthnBackend, AuthzBackend, UserId};

use crate::json::error::ApiError;
use crate::models::users::LoginUser;

#[derive(Clone)]
pub struct AuthGate;

#[async_trait]
impl AuthnBackend for AuthGate {
    type User = LoginUser;
    type Credentials = ();
    type Error = ApiError;

    async fn authenticate(&self, _: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        Ok(None)
    }

    async fn get_user(&self, _: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(None)
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