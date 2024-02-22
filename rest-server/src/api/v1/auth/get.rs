use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::app::auth::AuthSession;

pub async fn logout(
    mut auth_session: AuthSession
) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}