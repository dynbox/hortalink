use axum::Extension;
use axum::extract::Path;
use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::models::cart::Order;

pub async fn product(
    Extension(state): Extension<AppState>,
    Path(order_id): Path<i32>,
    auth_session: AuthSession,
) -> Result<(), ApiError> {
    let customer_id = Order::get_customer(&state.pool, order_id)
        .await?;

    if customer_id != auth_session.user.unwrap().id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()));
    }
    
    sqlx::query(
        r#"
            DELETE FROM cart
            WHERE id = $1
        "#
    )
        .bind(order_id)
        .execute(&state.pool)
        .await?;
    
    Ok(())
}