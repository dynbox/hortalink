use axum::{Extension, Json};
use axum::extract::Path;

use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::sellers::SellerResponse;
use crate::models::products::SellerProduct;
use crate::models::sellers::SellerUser;

pub async fn seller(
    Extension(state): Extension<AppState>,
    Path(seller_id): Path<i32>,
) -> Result<Json<SellerResponse>, ApiError> {
    let mut tx = state.pool.begin().await?;
    let seller = sqlx::query_as::<_, SellerUser>(
        r#"
            SELECT s.user_id AS id, u.avatar, 
                u.name, s.bio
            FROM sellers s
            JOIN users u ON s.user_id = u.id
            WHERE s.user_id = $1
            GROUP BY s.user_id, u.avatar, u.name, s.bio;
        "#
    )
        .bind(seller_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(ApiError::NotFound("Vendedor não encontrado".to_string()))?;

    let products = sqlx::query_as::<_, SellerProduct>(
        r#"
            SELECT s.id, p.id AS product_id, p.name, 
                s.photos, s.quantity, s.price,
                PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY sp.rating) AS rating
            FROM seller_products s
            JOIN products p ON s.product_id = p.id
            LEFT JOIN seller_product_ratings sp ON s.id = sp.seller_product_id
            WHERE s.seller_id = $1
            GROUP BY s.id, p.id, p.name, s.photos, s.quantity, s.price;
        "#
    )
        .bind(seller_id)
        .fetch_all(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(Json(SellerResponse { seller, products }))
}