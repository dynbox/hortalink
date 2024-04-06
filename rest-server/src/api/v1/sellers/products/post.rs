use axum::{Extension, Json};
use axum::extract::Path;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::products::PostSellerProduct;

pub async fn product(
    Extension(state): Extension<AppState>,
    Path(seller_id): Path<i32>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<PostSellerProduct>>,
) -> Result<(), ApiError> {
    if auth_session.user.unwrap().id != seller_id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()))
    }
    
    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await?;

    let product_id = sqlx::query_scalar::<_, i32>(
        r#"
            INSERT INTO seller_products (product_id, seller_id, price, quantity, photos)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        "#
    )
        .bind(payload.product_id)
        .bind(seller_id)
        .bind(payload.price)
        .bind(payload.quantity)
        .bind(payload.photos)
        .fetch_one(&mut *tx)
        .await?;

    if let Some(schedule_ids) = payload.schedules {
        let query = sqlx::query(
            r#"
                INSERT INTO product_schedules (seller_product_id, schedule_id)
                SELECT * FROM UNNEST ($1, $2)
            "#
        )
            .bind(vec![product_id].repeat(schedule_ids.len()))
            .bind(schedule_ids)
            .execute(&mut *tx)
            .await?;

        if query.rows_affected() == 0 {
            tx.rollback().await?;

            return Err(ApiError::Database("Falha ao adicionar agenda ao produto".to_string()));
        }
    }

    tx.commit().await?;
    Ok(())
}