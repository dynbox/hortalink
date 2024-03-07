use axum::{Extension, Json};
use axum::extract::Path;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::ratings::PatchSellerRating;

pub async fn rating(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    Path((_, rating_id)): Path<(i32, i32)>,
    WithValidation(payload): WithValidation<Json<PatchSellerRating>>,
) -> Result<(), ApiError> {
    let user = auth_session.user.unwrap();
    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await?;

    let query = sqlx::query(
        r#"
            UPDATE seller_rating
            SET rating = COALESCE($1, rating), 
            tags = COALESCE($2, tags)
            WHERE id = $3 AND author_id = $4
        "#
    )
        .bind::<Option<i16>>(payload.rating.map(|it| it.into()))
        .bind(payload.tags.map(|vec| {
            vec.into_iter()
                .map(|tag| tag.into())
                .collect::<Vec<i16>>()
        }))
        .bind(rating_id)
        .bind(user.id)
        .execute(&mut *tx)
        .await?;

    if query.rows_affected() == 0 {
        tx.rollback().await?;

        return Err(ApiError::Unauthorized("Somente o autor da avaliação pode editá-la".to_string()));
    }

    tx.commit().await?;
    Ok(())
}