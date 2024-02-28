use axum::{Extension, Json};
use axum::extract::Path;

use crate::app::web::AppState;
use crate::json::error::ApiError;
use crate::models::schedules::Schedule;

pub async fn schedules(
    Extension(state): Extension<AppState>,
    Path(seller_id): Path<i32>,
) -> Result<Json<Vec<Schedule>>, ApiError> {
    let schedules: Vec<Schedule> = sqlx::query_as::<_, Schedule>(
        r#"
            SELECT schedules.id, schedules.geolocation, 
                schedules.address, schedules.start_time, 
                schedules.end_time, schedules.day_of_week
            FROM schedules
            INNER JOIN seller_schedules ON schedules.id = seller_schedules.schedule_id
            WHERE seller_schedules.seller_id = $1
        "#,
    )
        .bind(seller_id)
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(schedules))
}