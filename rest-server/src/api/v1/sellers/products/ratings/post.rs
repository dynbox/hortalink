use axum::{Extension, Json};
use axum::extract::Path;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::ApiError;
use crate::json::ratings::PostSellerProductRating;

pub async fn rating(
    Extension(state): Extension<AppState>,
    Path((_, product_id)): Path<(i32, i32)>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<PostSellerProductRating>>,
) -> Result<(), ApiError> {
    let user_id = auth_session.user.unwrap().id;
    let payload = payload.into_inner();

    sqlx::query(
        r#"
            INSERT INTO seller_product_ratings (seller_product_id, customer_id, rating, content)
            VALUES ($1, $2, $3, $4)
        "#
    )
        .bind(product_id)
        .bind(user_id)
        .bind::<i16>(payload.rating.into())
        .bind(payload.content)
        .execute(&state.pool)
        .await?;

    Ok(())
}