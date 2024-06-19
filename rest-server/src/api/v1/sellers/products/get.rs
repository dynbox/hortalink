use axum::{Extension, Json};
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::products::SellerProductResponse;
use crate::models::products::SellerProduct;
use crate::models::schedules::Schedule;

pub async fn product(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path((_, product_id)): Path<(i32, i32)>,
) -> Result<Json<SellerProductResponse>, ApiError> {
    let mut tx = state.pool.begin().await?;

    let product = sqlx::query_as::<_, SellerProduct>(
        r#"
            SELECT s.id, p.id AS product_id, p.name, 
                s.photos, s.quantity, s.price,
                COALESCE(CAST(s.rating_sum AS FLOAT) / CAST(NULLIF(s.rating_quantity, 0) AS FLOAT), NULL) AS rating,
                s.rating_quantity, s.unit
            FROM seller_products s
            JOIN products p ON s.product_id = p.id
            LEFT JOIN seller_product_ratings sp ON s.id = sp.seller_product_id
            WHERE s.id = $1
            GROUP BY s.id, p.id, p.name, s.photos, s.quantity, s.price;
        "#
    )
        .bind(product_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(ApiError::NotFound("Produto não encontrado".to_string()))?;

    let schedule = sqlx::query_as::<_, Schedule>(
        r#"
            SELECT s.id, s.address, s.start_time, s.end_time, s.day_of_week
            FROM seller_products sp
            JOIN products_schedules sc ON sc.seller_product_id = sp.id
            JOIN schedules s ON sc.schedule_id = s.id
            WHERE sp.id = $1
        "#
    )
        .bind(product_id)
        .fetch_all(&mut *tx)
        .await?;

    if let Some(user) = auth_session.user {
        if user.roles.contains(&3) {
            sqlx::query(
                r#"
                INSERT INTO products_seen_recently (seller_product_id, customer)
                VALUES ($1, $2) 
                ON CONFLICT ON CONSTRAINT unique_product_customer
                DO UPDATE SET viewed_at = CURRENT_TIMESTAMP;
            "#
            )
                .bind(product_id)
                .bind(user.id)
                .execute(&mut *tx)
                .await?;
        }
    };
    
    tx.commit().await?;

    Ok(Json(SellerProductResponse { product, schedule }))
}