use axum::{Extension, Json};
use axum::extract::Path;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::ApiError;
use crate::json::ratings::PostSellerRating;

pub async fn rating(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path(seller_id): Path<i32>,
    WithValidation(payload): WithValidation<Json<PostSellerRating>>,
) -> Result<(), ApiError> {
    let user = auth_session.user.unwrap();
    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await?;

    sqlx::query(
        r#"
            INSERT INTO seller_rating (author_id, seller_id, rating, tags)
            VALUES ($1, $2, $3, $4)
        "#
    )
        .bind(user.id)
        .bind(seller_id)
        .bind::<i16>(payload.rating.into())
        .bind(payload.tags.map(|vec| {
            vec.into_iter()
                .map(|tag| tag.into())
                .collect::<Vec<i16>>()
        }))
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}