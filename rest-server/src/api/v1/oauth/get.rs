use axum::{Extension, Json};
use axum::extract::{Path, Query};
use axum::response::Redirect;
use axum_login::tower_sessions::Session;
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};
use reqwest::StatusCode;

use common::settings::Protocol;

use crate::app::auth::AuthSession;
use crate::app::provider::OAuthProvider;
use crate::app::server::AppState;
use crate::json::auth::{AuthzResp, UserInfo};
use crate::json::error::ApiError;
use crate::models::users::LoginUser;

pub async fn oauth_callback(
    session: Session,
    mut auth_session: AuthSession,
    Path(oauth_type): Path<String>,
    Extension(providers): Extension<OAuthProvider>,
    Query(AuthzResp { code, state: new_state }): Query<AuthzResp>,
    Extension(state): Extension<AppState>,
) -> Result<Redirect, ApiError> {
    /*
    if let Ok(Some(old_state)) = session.remove::<String>("oauth.csrf").await {
        if &old_state != new_state.secret() {
            return Err(ApiError::Custom(StatusCode::BAD_REQUEST, "cu".to_string()))
        };
    } else {
        return Err(ApiError::Custom(StatusCode::BAD_REQUEST, "falha state".to_string()))
    };
    
    let Ok(Some(pkce)) = session.remove::<String>("oauth.pkce").await else {
        return Err(ApiError::Custom(StatusCode::BAD_REQUEST, "falha".to_string()))
    };
    */
    
    let provider = providers.get_provider(&oauth_type);

    let token = if oauth_type == "linkedin" {
        provider.client
            .exchange_code(AuthorizationCode::new(code))
            .add_extra_param("client_secret", state.settings.secrets.linkedin.client_secret)
            //.set_pkce_verifier(PkceCodeVerifier::new(pkce))
            .request_async(&oauth2::reqwest::Client::builder()
                .redirect(oauth2::reqwest::redirect::Policy::limited(1))
                .build().unwrap())
            .await
            .unwrap()
            .access_token()
            .secret()
            .clone()
    } else {
        provider.get_token(code, /*PkceCodeVerifier::new(pkce)*/).await?
            .access_token()
            .secret()
            .clone()
    };

    let user_info: UserInfo = provider.get_user(&token).await;

    let user = sqlx::query_as::<_, LoginUser>(
        r#"
            SELECT
                id, password,
                roles, access_token
            FROM USERS
            WHERE email = $1
        "#
    )
        .bind(user_info.email)
        .fetch_optional(&state.pool)
        .await?;

    if let Some(user) = user {
        auth_session.login(&user).await?;
        
        Ok(Redirect::to(&format!("{}", state.settings.web.client.proxy_url())))
    } else {
        session.insert("oauth.token", token).await
            .map_err(|e| ApiError::Custom(StatusCode::INTERNAL_SERVER_ERROR, format!("Falha ao inserir na sessão: {e}")))?;
        
        Ok(Redirect::to(&format!("{}/access/signup?oauth_type={oauth_type}", state.settings.web.client.proxy_url())))
    }
}

pub async fn userinfo(
    session: Session,
    Path(oauth_type): Path<String>,
    Extension(providers): Extension<OAuthProvider>,
) -> Result<Json<UserInfo>, ApiError> {
    let provider = providers.get_provider(&oauth_type);
    let token = session.get::<String>("oauth.token").await;

    return if let Ok(Some(token)) = token {
        let user_info = provider.get_user(&token).await;
        Ok(Json(user_info))
    } else {
        Err(ApiError::NotFound("Oauth token not found".to_string()))
    };
}