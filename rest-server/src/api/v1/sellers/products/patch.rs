use axum::{Extension, Json};
use axum::extract::Path;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::products::PatchSellerProduct;
use crate::models::products::SellerProduct;

pub async fn product(
    Extension(state): Extension<AppState>,
    Path((seller_id, product_id)): Path<(i32, i32)>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<PatchSellerProduct>>,
) -> Result<(), ApiError> {
    let author = SellerProduct::get_author(&state.pool, product_id)
        .await?;

    if auth_session.user.unwrap().id != seller_id || author != seller_id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()));
    }

    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await?;

    sqlx::query(
        r#"
            UPDATE seller_products
            SET price = COALESCE($1, price), 
                quantity = COALESCE($2, quantity), 
                photos = COALESCE($3, photos)
            WHERE product_id = $4
        "#
    )
        .bind(payload.price)
        .bind(payload.quantity)
        .bind(payload.photos)
        .bind(product_id)
        .execute(&mut *tx)
        .await?;

    if let Some(remove_schedules) = payload.remove_schedules {
        sqlx::query(
            r#"
                DELETE FROM products_schedules
                WHERE seller_product_id = $1 AND schedule_id IN $2;
            "#
        )
            .bind(product_id)
            .bind(remove_schedules)
            .execute(&mut *tx)
            .await?;
    }

    if let Some(add_schedules) = payload.add_schedules {
        for schedule in add_schedules {
            sqlx::query(
                r#"
                INSERT INTO products_schedules (seller_product_id, schedule_id)
                VALUES ($1, $2)
            "#
            )
                .bind(product_id)
                .bind(schedule)
                .execute(&mut *tx)
                .await?;
        }
    }

    Ok(())
}