use axum::extract::State;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_garde::WithValidation;
use crate::app::backend::AuthSession;
use crate::app::web::AppState;
use crate::json::auth::{Credentials, LoginCreds, SignCreds};
use crate::models::users::ProtectedUser;

pub async fn login(
    mut auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<LoginCreds>>,
) -> impl IntoResponse {
    match auth_session.authenticate(Credentials::Password(payload.into_inner())).await {
        Ok(Some(user)) => {
            if auth_session.login(&user).await.is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            } else {
                StatusCode::OK.into_response()
            }
        }
        Ok(None) => StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn sign_in(
    State(state): State<AppState>,
    mut auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<SignCreds>>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, ProtectedUser>(
        r#"
            INSERT INTO users (name, email, password, username)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id, email, password,
                permissions, access_token,
                account_type
            "#
    )
        .bind(&payload.name)
        .bind(&payload.email)
        //.bind(&payload.phone.to_string())
        .bind(password_auth::generate_hash(&payload.password))
        .bind(&payload.username)
        .fetch_one(&state.pool)
        .await {
        Ok(user) => {
            if auth_session.login(&user).await.is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            } else {
                StatusCode::OK.into_response()
            }
        },
        Err(err) => {
            println!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}