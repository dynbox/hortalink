use axum::{Extension, Json};

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::models::cart::Order;

pub async fn orders(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = sqlx::query_as::<_, Order>(
        r#"
            SELECT c.id AS order_id, c.customer_id AS user_id, c.withdrawn, c.amount, 
                sp.price, u.name, u.avatar, sp.id AS product_id, sp.photos,
                p.name AS product_name
            FROM "cart" c
            JOIN "seller_products" sp ON c.seller_product_id = sp.id
            JOIN "products" p ON sp.product_id = p.id
            JOIN "users" u ON c.customer_id = u.id
            WHERE sp.seller_id = $1 AND c.status = 2 OR c.status = 3 or c.status = 4;
        "#
    )
        .bind(auth_session.user.unwrap().id)
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(orders))
}