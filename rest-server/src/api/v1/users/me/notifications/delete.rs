use axum::Extension;
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::ApiError;

pub async fn notification(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path(notification_id): Path<i32>,
) -> Result<(), ApiError> {
    let login_user = auth_session.user.unwrap();

    let mut tx = state.pool.begin().await?;
    let query = sqlx::query(
        r#"
        DELETE FROM notifications
        WHERE id = $1 AND user_id = $2
        "#,
    )
        .bind(notification_id)
        .bind(login_user.id)
        .execute(&mut *tx)
        .await?;

    if query.rows_affected() == 0 {
        tx.rollback().await?;
        
        return Err(ApiError::Unauthorized("Somente o autor da notificação pode excluí-la".to_string()));
    }

    tx.commit().await?;

    Ok(())
}