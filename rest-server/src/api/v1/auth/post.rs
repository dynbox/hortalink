use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use axum_garde::WithValidation;
use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::auth::{Credentials, LoginCreds, SignCreds};
use crate::json::error::error_message;
use crate::models::users::LoginUser;

pub async fn login(
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<LoginCreds>>,
) -> impl IntoResponse {
    match auth_session.authenticate(Credentials::Password(payload.into_inner())).await {
        Ok(Some(user)) => login_user(auth_session, user).await,
        Ok(None) => error_message(StatusCode::UNAUTHORIZED, "Usuário não encontrado"),
        Err(e) => error_message(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Falha ao autenticar usuário: {:?}", e)
        ),
    }
}

pub async fn sign_in(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<SignCreds>>,
) -> impl IntoResponse {
    let payload = payload.into_inner();

    match sqlx::query_as::<_, LoginUser>(
        r#"
            INSERT INTO users (name, username, email, role, avatar, password)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, password, role, access_token
        "#
    )
        .bind(payload.name)
        .bind(payload.username)
        .bind(payload.email)
        .bind(payload.role as i16)
        .bind(payload.avatar)
        .bind(password_auth::generate_hash(&payload.password))
        .fetch_one(&state.pool)
        .await {
        Ok(user) => login_user(auth_session, user).await,
        Err(e) => error_message(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Falha ao cadastrar usuário: {:?}", e)
        )
    }
}

async fn login_user(mut auth_session: AuthSession, user: LoginUser) -> Response {
    if let Err(e) = auth_session.login(&user).await {
        return error_message(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Falha ao logar: {}", e)
        );
    } else {
        StatusCode::OK.into_response()
    }
}