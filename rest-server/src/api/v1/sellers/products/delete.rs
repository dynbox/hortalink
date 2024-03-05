use axum::Extension;
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::ApiError;

pub async fn product(
    Extension(state): Extension<AppState>,
    Path((_, product_id)): Path<(i32, i32)>,
    auth_session: AuthSession,
) -> Result<(), ApiError> {
    let user = auth_session.user.unwrap();
    let mut tx = state.pool.begin().await?;

    sqlx::query(
        r#"
            DELETE FROM product_schedules
            WHERE seller_product_id = $1
        "#
    )
        .bind(product_id)
        .bind(user.id)
        .execute(&mut *tx)
        .await?;

    let query = sqlx::query(
        r#"
            DELETE FROM seller_products
            WHERE id = $1 AND seller_id = $2
        "#
    )
        .bind(product_id)
        .bind(user.id)
        .execute(&mut *tx)
        .await?;

    if query.rows_affected() == 0 {
        tx.rollback().await?;

        return Err(ApiError::Unauthorized("Somente o dono do produto pode excluí-lo".to_string()));
    }

    tx.commit().await?;
    Ok(())
}