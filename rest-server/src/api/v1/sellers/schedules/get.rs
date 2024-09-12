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
            SELECT sc.id, pl.address, sc.start_time, sc.end_time, sc.day_of_week
            FROM schedules sc
            JOIN places pl ON pl.id = sc.place
            WHERE seller_id = $1
        "#,
    )
        .bind(seller_id)
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(schedules))
}