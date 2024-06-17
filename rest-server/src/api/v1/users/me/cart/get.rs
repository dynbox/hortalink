use axum::{Extension, Json};

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::models::cart::Order;

pub async fn products(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
) -> Result<Json<Vec<Order>>, ApiError> {
    let cart_products = sqlx::query_as::<_, Order>(
        r#"
            SELECT c.id AS order_id, c.withdrawn, c.amount, 
                sp.price, u.name, u.avatar, sp.id AS product_id, sp.photos,
                sp.seller_id AS user_id, p.name AS product_name
            FROM cart c
            JOIN "seller_products" sp ON sp.id = c.seller_product_id
            JOIN "products" p ON sp.product_id = p.id
            JOIN "users" u ON u.id = sp.seller_id
            WHERE c.customer_id = $1;
        "#
    )
        .bind(auth_session.user.unwrap().id)
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(cart_products))
}