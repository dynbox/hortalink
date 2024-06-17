use axum::{Extension, Json};
use axum::extract::Path;
use serde::Serialize;

use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::models::products::SellerProduct;
use crate::models::sellers::PublicProfile;

#[derive(Serialize)]
pub struct UserResponse {
    profile: PublicProfile,
    products: Option<Vec<SellerProduct>>
}

pub async fn user(
    Extension(state): Extension<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, ApiError> {
    let profile = sqlx::query_as::<_, PublicProfile>(
        r#"
            SELECT 
                u.id, u.name, u.avatar, 4 = ANY(u.roles) as is_seller,
                s.bio, s.followers
            FROM "users" u
            LEFT JOIN "sellers" s ON u.id = s.user_id
            LEFT JOIN "customers" c ON u.id = c.user_id
            WHERE u.id = $1
        "#
    )
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(ApiError::NotFound("Usuário não encontrado".to_string()))?;

    if profile.is_seller {
        let products = sqlx::query_as::<_, SellerProduct>(
            r#"
                SELECT
                    sp.id, sp.price, sp.unit, sp.quantity,
                    sp.photos, (CAST(sp.rating_sum AS FLOAT) / CAST(sp.rating_quantity AS FLOAT)) AS rating,
                    sp.rating_quantity, p.id AS product_id, p.name
                FROM seller_products sp
                JOIN products p ON p.id = sp.product_id
                WHERE sp.seller_id = $1
            "#
        )
            .bind(user_id)
            .fetch_all(&state.pool)
            .await?;
        
        return Ok(Json(UserResponse { profile, products: Some(products) }))
    }
    
    Ok(Json(UserResponse { profile, products: None }))
}