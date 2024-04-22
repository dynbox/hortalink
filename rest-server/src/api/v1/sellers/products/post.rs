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
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()));
    }

    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await?;

    sqlx::query_scalar::<_, i32>(
        r#"
            INSERT INTO seller_products (product_id, seller_id, price, quantity, photos, schedule_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
        "#
    )
        .bind(payload.product_id)
        .bind(seller_id)
        .bind(payload.price)
        .bind(payload.quantity)
        .bind(payload.photos)
        .bind(payload.schedule_id)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}