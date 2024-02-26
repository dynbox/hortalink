use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::models::notifications::Notification;

pub async fn notifications(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession
) -> impl IntoResponse {
    let user_id = auth_session.user.unwrap().id;

    let notifications: Vec<Notification> = sqlx::query_as(
        r#"
            SELECT id, title, content, created_at, "read", 
                status, type as notification_type,
                icon
            FROM notifications
            WHERE user_id = $1
            ORDER BY created_at DESC
        "#,
    )
        .bind(user_id)
        .fetch_all(&state.pool)
        .await
        .unwrap();
    
    Json(notifications)
}