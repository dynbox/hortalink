use axum::Extension;
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::ApiError;
use crate::json::schedules::ScheduleParams;

pub async fn schedule(
    Extension(state): Extension<AppState>,
    Path(params): Path<ScheduleParams>,
    auth_session: AuthSession,
) -> Result<(), ApiError> {
    let login_user = auth_session.user.unwrap();
    let mut tx = state.pool.begin().await?;

    let is_author = sqlx::query_scalar::<_, bool>(
        r#"
            SELECT EXISTS (
                SELECT 1 FROM seller_schedules
                WHERE seller_id = $1 AND schedule_id = $2
            )
        "#
    )
        .bind(login_user.id)
        .bind(params.schedule_id)
        .fetch_one(&mut *tx)
        .await?;

    if !is_author {
        tx.rollback().await?;

        return Err(ApiError::Unauthorized("Somente o autor da agenda pode excluí-la".to_string()));
    }

    sqlx::query(r#"
        DELETE FROM seller_schedules
        WHERE schedule_id = $1
    "#)
        .bind(params.schedule_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(r#"
        DELETE FROM schedules
        WHERE id = $1
    "#)
        .bind(params.schedule_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}