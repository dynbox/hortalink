use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use tower_sessions::Session;
use crate::app::AppState;
use crate::json::auth::{AuthUrlResponse, Credentials, LoginCreds, SignCreds};
use crate::provider::AuthProviders;
use crate::routes::auth::backend::AuthSession;

pub async fn login(
    mut auth_session: AuthSession,
    Json(payload): Json<LoginCreds>,
) -> impl IntoResponse {
    match auth_session.authenticate(Credentials::Password(payload)).await {
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

pub async fn sign(
    auth_session: AuthSession,
    State(state): State<AppState>,
    Json(payload): Json<SignCreds>,
) -> impl IntoResponse {
    match sqlx::query("INSERT INTO users (name, email, phone, password) VALUES ($1, $2, $3, $4)")
        .bind(&payload.name).bind(&payload.email)
        .bind(&payload.phone.to_string())
        .bind(password_auth::generate_hash(&payload.password))
        .execute(&state.pool)
        .await {
        Ok(_) => {
            let creds = LoginCreds {
                email: payload.email,
                password: payload.password,
            };

            login(auth_session, Json(creds)).await.into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn oauth(
    Extension(providers): Extension<AuthProviders>,
    session: Session,
    Path(oauth_type): Path<String>,
) -> impl IntoResponse {
    let (auth_url, csrf) = providers.get_provider(&oauth_type)
        .unwrap()
        .auth_url();

    /*
    session
        .insert("oauth.csrf-state", csrf.secret())
        .await
        .expect("Serialization should not fail.");
     */

    let response = AuthUrlResponse {
        auth_url: if oauth_type == "linkedin" {
            auth_url.as_str().replace("+", "%20")
        } else {
            auth_url.as_str().to_string()
        }
    };

    Json(&response).into_response()
}