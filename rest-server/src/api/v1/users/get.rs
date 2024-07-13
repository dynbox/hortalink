use axum::{Extension, Json};
use axum::extract::{Path, Query};
use axum_garde::WithValidation;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::users::FilterUsers;
use crate::models::sellers::PublicProfile;
use crate::models::users::PreviewUser;

#[derive(Serialize)]
pub struct UserResponse {
    profile: PublicProfile,
}

pub async fn user(
    Extension(state): Extension<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, ApiError> {
    let profile = sqlx::query_as::<_, PublicProfile>(
        r#"
            SELECT 
                u.id, u.name, u.avatar, 4 = ANY(u.roles) as is_seller,
                s.bio, s.followers, c.following, c.orders_made, s.orders_received
            FROM users u
            LEFT JOIN sellers s ON u.id = s.user_id
            LEFT JOIN customers c ON u.id = c.user_id
            WHERE u.id = $1
        "#
    )
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(ApiError::NotFound("Usuário não encontrado".to_string()))?;
    
    if profile.is_seller {
        
    } else {
        
    }
    
    Ok(Json(UserResponse { profile }))
}

pub async fn users(
    Extension(state): Extension<AppState>,
    WithValidation(query): WithValidation<Query<FilterUsers>>,
) -> Result<Json<Vec<PreviewUser>>, ApiError> {
    let query = query.into_inner();

    if let Some(filter) = query.query {
        let users = sqlx::query_as::<_, PreviewUser>(
            &format!(
                r#"
            SELECT id AS user_id, avatar as user_avatar, name as user_name
            FROM users
            WHERE name LIKE '%{}%'
            LIMIT $1 OFFSET $2;
        "#, filter.to_lowercase()
            )
        )
            .bind(query.per_page)
            .bind((query.page - 1) * query.per_page)
            .fetch_all(&state.pool)
            .await?;

        Ok(Json(users))
    } else {
        let users = recommendations(&state.pool)
            .await?;

        Ok(Json(users))
    }
}

pub async fn recommendations(pool: &Pool<Postgres>)  -> Result<Vec<PreviewUser>, ApiError> {
    Ok(sqlx::query_as::<_, PreviewUser>(
        r#"
                SELECT s.user_id, u.avatar as user_avatar, u.name as user_name
                FROM sellers s
                JOIN (
                    SELECT DISTINCT seller_id
                    FROM seller_products
                ) sp ON s.user_id = sp.seller_id
                JOIN users u ON u.id = s.user_id
                ORDER BY RANDOM()
                LIMIT 9;
            "#
    )
        .fetch_all(pool)
        .await?)
}