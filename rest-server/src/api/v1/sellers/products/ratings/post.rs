use axum::{Extension, Json};
use axum::extract::Path;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::ratings::PostSellerProductRating;

pub async fn rating(
    Extension(state): Extension<AppState>,
    Path((seller_id, product_id)): Path<(i32, i32)>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<PostSellerProductRating>>,
) -> Result<(), ApiError> {
    let login_user_id = auth_session.user.unwrap().id;

    if login_user_id == seller_id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()))
    }
    
    let payload = payload.into_inner();

    sqlx::query(
        r#"
            INSERT INTO seller_product_ratings (seller_product_id, author_id, rating, content)
            VALUES ($1, $2, $3, $4)
        "#
    )
        .bind(product_id)
        .bind(login_user_id)
        .bind::<i16>(payload.rating.into())
        .bind(payload.content)
        .execute(&state.pool)
        .await?;

    Ok(())
}