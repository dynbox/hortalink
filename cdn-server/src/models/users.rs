use axum_login::AuthUser;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct LoginUser {
    pub id: i32,
    pub password: Option<String>,
    pub roles: Vec<i16>,
    access_token: Option<String>,
}

impl AuthUser for LoginUser {
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