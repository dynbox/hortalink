use axum::Extension;
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use http::StatusCode;
use oauth2::{AuthorizationCode, TokenResponse};
use oauth2::reqwest::async_http_client;
use crate::app::backend::AuthSession;
use crate::app::provider::AuthProviders;
use crate::app::web::AppState;
use crate::json::auth::{AuthzResp, Credentials, OAuthCreds};

pub async fn oauth_callback(
    mut auth_session: AuthSession,
    Path(oauth_type): Path<String>,
    Extension(providers): Extension<AuthProviders>,
    Query(AuthzResp { code, state: _ }): Query<AuthzResp>,
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