use axum::{Extension, Json};
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::ApiError;
use crate::json::notifications::UpdateNotificationPayload;

pub async fn read(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path(notification_id): Path<i32>,
    Json(payload): Json<UpdateNotificationPayload>,
) -> Result<(), ApiError> {
    let login_user = auth_session.user.unwrap();

    let mut tx = state.pool.begin().await?;
    let query = sqlx::query(r#"
        UPDATE notifications
        SET read = $2
        WHERE id = $1 AND user_id = $3
    "#)
        .bind(notification_id)
        .bind(payload.read)
        .bind(login_user.id)
        .execute(&mut *tx)
        .await?;

    if query.rows_affected() == 0 {
        tx.rollback().await?;

        return Err(ApiError::Unauthorized("Somente o autor da notificação pode editá-la".to_string()));
    }

    tx.commit().await?;
    Ok(())
}