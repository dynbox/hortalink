use axum::{Extension, Json};
use axum::extract::Path;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::products::PatchSellerProduct;

pub async fn product(
    Extension(state): Extension<AppState>,
    Path((_, product_id)): Path<(i32, i32)>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<PatchSellerProduct>>,
) -> Result<(), ApiError> {
    let payload = payload.into_inner();
    let user = auth_session.user.unwrap();
    let mut tx = state.pool.begin().await?;

    let query = sqlx::query(
        r#"
            UPDATE seller_products
            SET price = COALESCE($1, price), 
                quantity = COALESCE($2, quantity), 
                photos = COALESCE($3, photos)
            WHERE product_id = $4 AND seller_id = $5
        "#
    )
        .bind(payload.price)
        .bind(payload.quantity)
        .bind(payload.photos)
        .bind(product_id)
        .bind(user.id)
        .execute(&mut *tx)
        .await?;

    if query.rows_affected() == 0 {
        tx.rollback().await?;
        return Err(ApiError::Unauthorized("Somente o dono do produto pode editá-lo".to_string()));
    }

    if let Some(remove_schedules) = payload.remove_schedules {
        sqlx::query(
            r#"
                DELETE FROM product_schedules
                WHERE schedule_id = ANY($1)
            "#
        )
            .bind(remove_schedules)
            .execute(&mut *tx)
            .await?;
    }
    
    if let Some(add_schedules) = payload.add_schedules {
        sqlx::query(
            r#"
                INSERT INTO product_schedules (seller_product_id, schedule_id)
                SELECT * FROM UNNEST ($1, $2)
            "#
        )
            .bind(vec![product_id].repeat(add_schedules.len()))
            .bind(add_schedules)
            .execute(&mut *tx)
            .await?;
    }

    Ok(())
}