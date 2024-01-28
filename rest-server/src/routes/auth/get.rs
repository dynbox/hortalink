use axum::Extension;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use oauth2::{AuthorizationCode, TokenResponse};
use oauth2::reqwest::async_http_client;
use tower_sessions::Session;
use crate::app::AppState;
use crate::json::auth::{AuthzResp, Credentials, OAuthCreds};
use crate::provider::AuthProviders;
use crate::routes::auth::backend::AuthSession;

pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn oauth_callback(
    mut auth_session: AuthSession,
    session: Session,
    Path(oauth_type): Path<String>,
    Extension(providers): Extension<AuthProviders>,
    Query(AuthzResp { code, state: csrf }): Query<AuthzResp>,
    State(state): State<AppState>
) -> Response {
    /*
    if let Some(saved_csrf) = session.remove::<String>("oauth.csrf-state").await.unwrap() {
        if csrf.secret().clone() != saved_csrf {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    } else {
        return StatusCode::UNAUTHORIZED.into_response();
    }
     */

    let provider = providers.get_provider(&oauth_type)
        .unwrap();

    let token = if oauth_type == "linkedin" {
        provider.client
            .exchange_code(AuthorizationCode::new(code))
            .add_extra_param("client_secret", state.settings.secrets.linkedin.client_secret)
            .request_async(async_http_client)
            .await
            .unwrap()
            .access_token()
            .secret()
            .clone()
    } else {
        provider.get_token(code).await
            .access_token()
            .secret()
            .clone()
    };

    let user_info = provider.get_user(&token).await;

    let creds = Credentials::OAuth(OAuthCreds { user: user_info, token });

    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to(&format!("{}{}", state.settings.webapp.protocol_url(), "/")).into_response()
}