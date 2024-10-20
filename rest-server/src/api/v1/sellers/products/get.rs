use axum::{Extension, Json};
use axum::extract::Path;

use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::products::SellerProductResponse;
use crate::models::products::SellerProduct;
use crate::models::schedules::Schedule;

pub async fn product(
    Extension(state): Extension<AppState>,
    Path((seller_id, product_id)): Path<(i32, i32)>,
) -> Result<Json<SellerProductResponse>, ApiError> {
    let mut tx = state.pool.begin().await?;

    let product = sqlx::query_as::<_, SellerProduct>(
        r#"
            SELECT s.id, p.id AS product_id, p.name, 
                s.photos, s.quantity, s.price,
                PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY sp.rating) AS rating
            FROM seller_products s
            JOIN products p ON s.product_id = p.id
            LEFT JOIN seller_product_ratings sp ON s.id = sp.seller_product_id
            WHERE s.id = $1 AND seller_id = $2
            GROUP BY s.id, p.id, p.name, s.photos, s.quantity, s.price;
        "#
    )
        .bind(product_id)
        .bind(seller_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(ApiError::NotFound("Produto não encontrado".to_string()))?;

    let schedule = sqlx::query_as::<_, Schedule>(
        r#"
            SELECT s.id, s.address, s.start_time, s.end_time, s.day_of_week
            FROM seller_products sp
            JOIN schedules s ON sp.schedule_id = s.id
            WHERE sp.id = $1
        "#
    )
        .bind(product_id)
        .fetch_all(&mut *tx)
        .await?;

    Ok(Json(SellerProductResponse { product, schedule }))
}