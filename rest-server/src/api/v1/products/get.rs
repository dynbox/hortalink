use axum::{Extension, Json};
use axum::extract::Query;
use axum_garde::WithValidation;

use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::products::FilterProducts;
use crate::models::products::SellerProductPreview;

pub async fn filter_products(
    Extension(state): Extension<AppState>,
    WithValidation(query): WithValidation<Query<FilterProducts>>,
) -> Result<Json<Vec<SellerProductPreview>>, ApiError> {
    let query = query.into_inner();
    let mut sql_query = String::from(
        r#"
            SELECT s.id, p.id AS product_id, p.name,
               s.photos, s.price,
               COALESCE(s.rating_sum / NULLIF(s.rating_quantity, 0), NULL) AS rating,
               s.rating_quantity
        "#
    );

    if let (Some(latitude), Some(longitude)) = (query.latitude, query.longitude) {
        sql_query.push_str(&format!(", ST_Distance(sc.geolocation, ST_SetSRID(ST_MakePoint({longitude}, {latitude}),4326)) AS dist"));
    }

    sql_query.push_str(r#"
            FROM seller_products s
            JOIN products p ON s.product_id = p.id
            JOIN products_schedules ps ON ps.seller_product_id = s.id
            JOIN schedules sc ON sc.id = ps.schedule_id
            WHERE 1 = 1
    "#);

    if let Some(max_price) = query.max_price {
        sql_query.push_str(&format!("AND s.price < {max_price} "));
    }

    if let Some(min_price) = query.min_price {
        sql_query.push_str(&format!("AND s.price > {min_price} "));
    }

    if let Some(min_stars) = query.min_stars {
        sql_query.push_str(&format!("AND rating >= {} ", min_stars as i16));
    }

    if let Some(product_type) = query.product_type {
        sql_query.push_str(&format!("AND p.id = {product_type} "));
    }

    if let Some(start_time) = query.start_time {
        sql_query.push_str(&format!("AND sc.start_time > {start_time} "));
    }

    if let Some(day_of_week) = query.day_of_week {
        sql_query.push_str(&format!("AND sc.day_of_week = {} ", day_of_week as i16));
    }

    if let (Some(latitude), Some(longitude)) = (query.latitude, query.longitude) {
        sql_query.push_str(&format!("AND ST_DWithin(sc.geolocation, ST_MakePoint({longitude}, {latitude})::geography, 45000) "));
    }

    sql_query.push_str(&format!("LIMIT {} OFFSET {}", query.per_page, (query.page - 1) * query.per_page));

    let products = sqlx::query_as::<_, SellerProductPreview>(&sql_query)
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(products))
}