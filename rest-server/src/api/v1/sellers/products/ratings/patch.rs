use axum::{Extension, Json};
use axum::extract::Path;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::ratings::PatchSellerProductRating;

pub async fn rating(
    Extension(state): Extension<AppState>,
    Path((seller_id, _, rating_id)): Path<(i32, i32, i32)>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<PatchSellerProductRating>>,
) -> Result<(), ApiError> {
    let login_user_id = auth_session.user.unwrap().id;

    if login_user_id == seller_id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()))
    }

    let payload = payload.into_inner();
    
    sqlx::query(
        r#"
            UPDATE seller_product_ratings SET 
            rating = COALESCE($1, rating),
            content = COALESCE($2, content)
            WHERE id = $3 AND customer_id = $4
        "#
    )
        .bind::<Option<i16>>(payload.rating.map(|it| it.into()))
        .bind(payload.content)
        .bind(rating_id)
        .bind(login_user_id)
        .execute(&state.pool)
        .await?;

    Ok(())
}