use axum::Extension;
use axum::extract::Path;
use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn rating(
    Extension(state): Extension<AppState>,
    Path((_, _, rating_id)): Path<(i32, i32, i32)>,
    auth_session: AuthSession,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
            DELETE FROM seller_product_ratings
            WHERE id = $1 AND customer_id = $2
        "#
    )
        .bind(rating_id)
        .bind(auth_session.user.unwrap().id)
        .execute(&state.pool)
        .await?;
    
    Ok(())
}