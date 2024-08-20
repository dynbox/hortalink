use axum::{Extension, Json};
use axum::extract::Query;
use axum_garde::WithValidation;
use sqlx::{Pool, Postgres};

use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::utils::HomePage;
use crate::models::products::SellerProductPreview;

pub async fn more_orders(
    Extension(state): Extension<AppState>,
    WithValidation(query): WithValidation<Query<HomePage>>,
) -> Result<Json<Vec<SellerProductPreview>>, ApiError> {
    Ok(Json(fetch(query.into_inner(), &state.pool).await?))
}

pub async fn fetch(
    query: HomePage,
    pool: &Pool<Postgres>,
) -> Result<Vec<SellerProductPreview>, ApiError> {
    let more_orders = sqlx::query_as::<_, SellerProductPreview>(
        r#"
            SELECT DISTINCT ON (sp.id) sp.id, p.id AS product_id, p.name,
                sp.photos, sp.price,
                COALESCE(CAST(sp.rating_sum AS FLOAT) / CAST(NULLIF(sp.rating_quantity, 0) AS FLOAT), NULL) AS rating,
                sp.rating_quantity, sp.unit,
                (CASE
                    WHEN $1 IS NULL OR $2 IS NULL THEN NULL
                    ELSE ST_Distance(s.geolocation, ST_SetSRID(ST_MakePoint($1, $2),4674))
                END) AS dist, sp.seller_id
            FROM cart c
            LEFT JOIN seller_products sp ON c.seller_product_id = sp.id
            JOIN products p ON sp.product_id = p.id
            JOIN products_schedules ps ON ps.seller_product_id = sp.id
            JOIN schedules s ON s.id = ps.schedule_id
            WHERE status = 4
            GROUP BY sp.id, p.id, s.geolocation
            ORDER BY sp.id
            LIMIT $3 OFFSET $4
        "#
    )
        .bind(query.longitude)
        .bind(query.latitude)
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(pool)
        .await?;

    Ok(more_orders)
}