use axum::Extension;
use axum::response::IntoResponse;
use crate::app::auth::AuthSession;
use crate::app::web::AppState;

pub async fn notifications(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    todo!()
}