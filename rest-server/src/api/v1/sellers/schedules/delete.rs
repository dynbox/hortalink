use axum::Extension;
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::ApiError;

pub async fn schedule(
    Extension(state): Extension<AppState>,
    Path((_, schedule_id)): Path<(i32, i32)>,
    auth_session: AuthSession,
) -> Result<(), ApiError> {
    let login_user = auth_session.user.unwrap();
    let mut tx = state.pool.begin().await?;

    let query = sqlx::query(r#"
        DELETE FROM seller_schedules
        WHERE schedule_id = $1 AND seller_id = $2
    "#)
        .bind(schedule_id)
        .bind(login_user.id)
        .execute(&mut *tx)
        .await?;

    if query.rows_affected() == 0 {
        tx.rollback().await?;

        return Err(ApiError::Unauthorized("Somente o autor da agenda pode excluí-la".to_string()));
    }

    sqlx::query(r#"
        DELETE FROM product_schedules
        WHERE schedule_id = $1
    "#)
        .bind(schedule_id)
        .bind(login_user.id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(r#"
        DELETE FROM schedules
        WHERE id = $1
    "#)
        .bind(schedule_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}