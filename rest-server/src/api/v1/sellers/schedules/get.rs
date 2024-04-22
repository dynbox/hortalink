use axum::{Extension, Json};
use axum::extract::Path;

use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::models::schedules::Schedule;

pub async fn schedules(
    Extension(state): Extension<AppState>,
    Path(seller_id): Path<i32>,
) -> Result<Json<Vec<Schedule>>, ApiError> {
    let schedules: Vec<Schedule> = sqlx::query_as::<_, Schedule>(
        r#"
            SELECT id, address, start_time, end_time, day_of_week
            FROM schedules
            WHERE seller_id = $1
        "#,
    )
        .bind(seller_id)
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(schedules))
}