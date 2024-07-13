use axum::{Extension, Json};
use axum::extract::Query;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::utils::Location;
use crate::models::cart::Order;

pub async fn products(
    Extension(state): Extension<AppState>,
    WithValidation(query): WithValidation<Query<Location>>,
    auth_session: AuthSession,
) -> Result<Json<Vec<Order>>, ApiError> {
    let mut sql_query = r#"
        SELECT c.id AS order_id, c.withdrawn, c.amount,
            sp.price, u.name, u.avatar, sp.id AS product_id, sp.photos,
            sp.seller_id AS user_id, p.name AS product_name,
            sp.unit
        "#.to_string();

    if let (Some(latitude), Some(longitude)) = (query.latitude, query.longitude) {
        sql_query.push_str(&format!(", ST_Distance(sc.geolocation, ST_SetSRID(ST_MakePoint({longitude}, {latitude}),4674)) AS dist "));
    } else {
        sql_query.push_str(", null AS dist ");
    }

    sql_query.push_str(
        r#"
            FROM cart c
            JOIN seller_products sp ON sp.id = c.seller_product_id
            JOIN products p ON sp.product_id = p.id
            JOIN users u ON u.id = sp.seller_id
            JOIN products_schedules ps ON ps.seller_product_id = sp.id 
            JOIN schedules sc ON sc.id = ps.schedule_id
            WHERE c.customer_id = $1
        "#
    );

    let cart_products = sqlx::query_as::<_, Order>(&sql_query)
        .bind(auth_session.user.unwrap().id)
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(cart_products))
}