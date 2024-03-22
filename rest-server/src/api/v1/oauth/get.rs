use axum::Extension;
use axum::extract::{Path, Query};
use axum::response::Redirect;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, TokenResponse};
use common::settings::Protocol;
use crate::app::auth::AuthSession;
use crate::app::provider::OAuthProvider;
use crate::app::server::AppState;
use crate::json::auth::{AuthzResp, Credentials, OAuthCreds};
use crate::json::error::ApiError;

pub async fn oauth_callback(
    mut auth_session: AuthSession,
    Path(oauth_type): Path<String>,
    Extension(providers): Extension<OAuthProvider>,
    Query(AuthzResp { code, state: _ }): Query<AuthzResp>,
    Extension(state): Extension<AppState>
) -> Result<Redirect, ApiError> {
    /*
    if let Some(saved_csrf) = session.remove::<String>("oauth.csrf-state").await.unwrap() {
        if csrf.secret().clone() != saved_csrf {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    } else {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    */

    let provider = providers.get_provider(&oauth_type);

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
            .map_err(|_| ())?
            .access_token()
            .secret()
            .clone()
    };

    let user_info = provider.get_user(&token).await;

    let creds = Credentials::OAuth(OAuthCreds { user: user_info, token });

    let user = auth_session.authenticate(creds).await?
        .ok_or(ApiError::NotFound("Usuário não encontrado".to_string()))?;

    auth_session.login(&user).await?;

    Ok(Redirect::to(&format!("{}{}", state.settings.web.client.protocol_url(), "/")))
}