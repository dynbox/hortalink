use axum::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use crate::app::auth::AuthSession;
use crate::app::web::AppState;

pub async fn mark_as_read(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path(notification_id): Path<u32>,
) -> impl IntoResponse {
    todo!()
}