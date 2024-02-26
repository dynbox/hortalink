use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::error_message;
use crate::json::notifications::UpdateNotificationPayload;

pub async fn read(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path(notification_id): Path<i32>,
    Json(payload): Json<UpdateNotificationPayload>,
) -> impl IntoResponse {
    let login_user = auth_session.user.unwrap();
    
    let mut tx = state.pool.begin().await.unwrap();
    let query = sqlx::query(r#"
        UPDATE notifications
        SET read = $2
        WHERE id = $1 AND user_id = $3
    "#)
        .bind(notification_id)
        .bind(payload.read)
        .bind(login_user.id)
        .execute(&mut *tx)
        .await
        .unwrap();

    if query.rows_affected() ==  0 {
        tx.rollback().await.unwrap();
        
        return error_message(
            StatusCode::UNAUTHORIZED,
            "Você não é o dono da notificação ou a notificação não existe",
        );
    }

    tx.commit().await.unwrap();
    StatusCode::OK.into_response()
}