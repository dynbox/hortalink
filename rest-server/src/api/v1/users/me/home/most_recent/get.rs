use axum::{Extension, Json};
use axum::extract::Query;
use axum_garde::WithValidation;
use sqlx::{Pool, Postgres};

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::utils::HomePage;
use crate::models::products::SellerProductPreview;

pub async fn viewed_recently(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    WithValidation(query): WithValidation<Query<HomePage>>,
) -> Result<Json<Vec<SellerProductPreview>>, ApiError> {
    let user = auth_session.user.unwrap();

    Ok(Json(fetch(user.id, query.into_inner(), &state.pool).await?))
}

pub async fn fetch(
    id: i32,
    query: HomePage,
    pool: &Pool<Postgres>,
) -> Result<Vec<SellerProductPreview>, ApiError> {
    let seen_recently = sqlx::query_as::<_, SellerProductPreview>(
        r#"
                SELECT DISTINCT ON (sp.id) sp.id, p.id AS product_id, p.name,
                    sp.photos, sp.price,
                    COALESCE(CAST(sp.rating_sum AS FLOAT) / CAST(NULLIF(sp.rating_quantity, 0) AS FLOAT), NULL) AS rating,
                    sp.rating_quantity, sp.unit,
                    (CASE
                        WHEN $1 IS NULL OR $2 IS NULL THEN NULL
                        ELSE ST_Distance(s.geolocation, ST_SetSRID(ST_MakePoint($1, $2),4674))
                    END) AS dist, sp.seller_id
                FROM products_seen_recently sr
                LEFT JOIN seller_products sp ON sp.id = sr.seller_product_id
                JOIN products p ON sp.product_id = p.id
                JOIN products_schedules ps ON ps.seller_product_id = sp.id
                JOIN schedules s ON s.id = ps.schedule_id
                WHERE sr.customer = $3
                ORDER BY sp.id, sr.viewed_at DESC
                LIMIT $4 OFFSET $5;
            "#
    )
        .bind(query.longitude)
        .bind(query.latitude)
        .bind(id)
        .bind(query.per_page)
        .bind((query.page - 1) * query.per_page)
        .fetch_all(pool)
        .await?;

    Ok(seen_recently)
}