use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::error_message;
use crate::json::schedules::ScheduleParams;

pub async fn schedule(
    Extension(state): Extension<AppState>,
    Path(params): Path<ScheduleParams>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    let login_user = auth_session.user.unwrap();
    let mut tx = state.pool.begin().await.unwrap();

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
        .await
        .unwrap();

    if !is_author {
        tx.rollback().await.unwrap();

        return error_message(
            StatusCode::UNAUTHORIZED,
            "Você não é o dono da notificação ou a notificação não existe",
        );
    }

    sqlx::query(r#"
        DELETE FROM seller_schedules
        WHERE schedule_id = $1
    "#)
        .bind(params.schedule_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    sqlx::query(r#"
        DELETE FROM schedules
        WHERE id = $1
    "#)
        .bind(params.schedule_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();
    StatusCode::OK.into_response()
}