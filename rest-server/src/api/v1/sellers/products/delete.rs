use axum::Extension;
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn product(
    Extension(state): Extension<AppState>,
    Path((seller_id, product_id)): Path<(i32, i32)>,
    auth_session: AuthSession,
) -> Result<(), ApiError> {
    if auth_session.user.unwrap().id != seller_id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()))
    }
    
    let mut tx = state.pool.begin().await?;

    sqlx::query(
        r#"
            DELETE FROM product_schedules
            WHERE seller_product_id = $1
        "#
    )
        .bind(product_id)
        .bind(seller_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        r#"
            DELETE FROM seller_products
            WHERE id = $1 AND seller_id = $2
        "#
    )
        .bind(product_id)
        .bind(seller_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}