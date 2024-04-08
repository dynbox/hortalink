use axum::{Extension, Json};
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::cart::PostProductCart;
use crate::json::error::ApiError;

pub async fn product(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<PostProductCart>>,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
            INSERT INTO cart (seller_product_id, customer_id, withdrawn, amount)
            VALUES ($1, $2, $3, $4)
        "#
    )
        .bind(payload.seller_product_id)
        .bind(auth_session.user.unwrap().id)
        .bind(payload.withdrawn)
        .bind(payload.amount)
        .execute(&state.pool)
        .await?;

    Ok(())
}