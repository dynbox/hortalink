use axum::Extension;
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn rating(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path((_, rating_id)): Path<(i32, i32)>,
) -> Result<(), ApiError> {
    let user = auth_session.user.unwrap();
    let mut tx = state.pool.begin().await?;

    let query = sqlx::query(
        r#"
            DELETE FROM seller_rating
            WHERE id = $1 AND author_id = $2
        "#
    )
        .bind(rating_id)
        .bind(user.id)
        .execute(&mut *tx)
        .await?;

    if query.rows_affected() == 0 {
        tx.rollback().await?;
        
        return Err(ApiError::Unauthorized("Somente o autor da avaliação pode excluí-la".to_string()));
    }

    tx.commit().await?;
    Ok(())
}