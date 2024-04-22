use axum::{Extension, Json};
use axum::extract::Query;
use axum_garde::WithValidation;
use sqlx::QueryBuilder;

use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::products::FilterProducts;
use crate::models::products::SellerProduct;

pub async fn filter_products(
    Extension(state): Extension<AppState>,
    WithValidation(query): WithValidation<Query<FilterProducts>>,
) -> Result<Json<Vec<SellerProduct>>, ApiError> {
    let query = query.into_inner();
    let mut sql_query = QueryBuilder::<sqlx::Postgres>::new(
        r#"
        "#
    );

    if let Some(max_price) = query.max_price {
        sql_query.push("AND sp.price < ");
        sql_query.push_bind(max_price);
    }

    if let Some(min_price) = query.min_price {
        sql_query.push("AND sp.price > ");
        sql_query.push_bind(min_price);
    }

    if let Some(min_stars) = query.min_stars {
        sql_query.push("AND rating > ");
        sql_query.push_bind(min_stars as i16);
    }

    if let Some(product_type) = query.product_type {
        sql_query.push("AND p.id = ");
        sql_query.push_bind(product_type);
    }

    if let Some(start_time) = query.start_time {
        sql_query.push("AND sc.start_time > ");
        sql_query.push_bind(start_time);
    }

    if let Some(day_of_week) = query.day_of_week {
        sql_query.push("AND sc.day_of_week = ");
        sql_query.push_bind(day_of_week as i16);
    }

    if let (Some(latitude), Some(longitude)) = (query.max_price, query.longitude) {
        sql_query.push(&format!("AND ST_DWithin(sc.geolocation, 'POINT({longitude} {latitude})'::geography, 45000) "));
    }

    sql_query.push(&format!("LIMIT {} OFFSET {}", query.per_page, (query.page - 1) * query.per_page));

    let products = sqlx::query_as::<_, SellerProduct>(&sql_query.into_sql())
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(products))
}