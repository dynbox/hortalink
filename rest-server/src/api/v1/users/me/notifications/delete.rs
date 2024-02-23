use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::error_message;

pub async fn notification(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path(notification_id): Path<i32>,
) -> impl IntoResponse {
    let login_user = auth_session.user.unwrap();
    
    let mut tx = state.pool.begin().await.unwrap();
    let query = sqlx::query(
        r#"
        DELETE FROM notifications
        WHERE id = $1 AND user_id = $2
        "#,
    )
        .bind(notification_id)
        .bind(login_user.id)
        .execute(&mut *tx)
        .await
        .unwrap();

    if query.rows_affected() ==  0 {
        return error_message(
            StatusCode::UNAUTHORIZED,
            "Você não é o dono da notificação ou a notificação não existe",
        );
    }
    
    tx.commit().await.unwrap();
    StatusCode::OK.into_response()
}