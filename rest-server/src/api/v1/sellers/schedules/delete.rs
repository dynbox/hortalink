use axum::Extension;
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::models::schedules::Schedule;

pub async fn schedule(
    Extension(state): Extension<AppState>,
    Path((seller_id, schedule_id)): Path<(i32, i32)>,
    auth_session: AuthSession,
) -> Result<(), ApiError> {
    let author = Schedule::get_author(&state.pool, schedule_id)
        .await?;

    if auth_session.user.unwrap().id != seller_id || author != seller_id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()));
    }

    let mut tx = state.pool.begin().await?;

    sqlx::query(
        r#"
            DELETE FROM schedules
            WHERE id = $1
        "#
    )
        .bind(schedule_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}