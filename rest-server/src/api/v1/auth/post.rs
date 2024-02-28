use axum::{Extension, Json};
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::auth::{Credentials, LoginCreds, SignCreds};
use crate::json::error::ApiError;
use crate::models::users::LoginUser;

pub async fn login(
    mut auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<LoginCreds>>,
) -> Result<(), ApiError> {
    match auth_session.authenticate(Credentials::Password(payload.into_inner())).await? {
        Some(user) => {
            auth_session.login(&user).await?;

            Ok(())
        }
        None => Err(ApiError::Unauthorized("Usuário não encontrado".to_string())),
    }
}

pub async fn sign_in(
    Extension(state): Extension<AppState>,
    mut auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<SignCreds>>,
) -> Result<(), ApiError> {
    let payload = payload.into_inner();

    let user = sqlx::query_as::<_, LoginUser>(
        r#"
            INSERT INTO users (name, username, email, roles, avatar, password)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, password, roles, access_token
        "#
    )
        .bind(payload.name)
        .bind(payload.username)
        .bind(payload.email)
        .bind(vec![payload.role as i16])
        .bind(payload.avatar)
        .bind(password_auth::generate_hash(&payload.password))
        .fetch_one(&state.pool)
        .await?;

    auth_session.login(&user).await?;
    Ok(())
}