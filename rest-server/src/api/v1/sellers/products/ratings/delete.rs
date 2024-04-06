use axum::Extension;
use axum::extract::Path;
use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn rating(
    Extension(state): Extension<AppState>,
    Path((seller_id, _, rating_id)): Path<(i32, i32, i32)>,
    auth_session: AuthSession,
) -> Result<(), ApiError> {
    let login_user_id = auth_session.user.unwrap().id;

    if login_user_id == seller_id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()))
    }
    
    sqlx::query(
        r#"
            DELETE FROM seller_product_ratings
            WHERE id = $1 AND customer_id = $2
        "#
    )
        .bind(rating_id)
        .bind(login_user_id)
        .execute(&state.pool)
        .await?;
    
    Ok(())
}