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
            SELECT u.name, u.avatar, sp.seller_id AS user_id,
                   json_agg(json_build_object(
                       'order_id', c.id,
                       'withdrawn', c.withdrawn,
                       'amount', c.amount,
                       'price', sp.price,
                       'product_id', sp.id,
                       'photos', sp.photos,
                       'product_name', p.name,
                       'unit', sp.unit,
        "#.to_string();

    if let (Some(latitude), Some(longitude)) = (query.latitude, query.longitude) {
        sql_query.push_str(&format!("'dist', ST_DistanceSphere(sc.geolocation, ST_SetSRID(ST_MakePoint({longitude}, {latitude}),4674)) "));
    } else {
        sql_query.push_str("'dist', null");
    }
    
    sql_query.push_str(")) AS products");

    sql_query.push_str(
        r#"
            FROM cart c
            JOIN seller_products sp ON sp.id = c.seller_product_id
            JOIN products p ON sp.product_id = p.id
            JOIN users u ON u.id = sp.seller_id
            LEFT JOIN (
                SELECT DISTINCT (seller_product_id) schedule_id, seller_product_id
                FROM products_schedules
            ) ps ON ps.seller_product_id = sp.id
            JOIN schedules sc ON sc.id = ps.schedule_id
            WHERE c.customer_id = $1
            GROUP BY u.name, u.avatar, sp.seller_id
        "#
    );

    let cart_products = sqlx::query_as::<_, Order>(&sql_query)
        .bind(auth_session.user.unwrap().id)
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(cart_products))
}