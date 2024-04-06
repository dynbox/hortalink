use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::schedules::CreateSchedulePayload;

pub async fn schedule(
    Extension(state): Extension<AppState>,
    Path(seller_id): Path<i32>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<CreateSchedulePayload>>,
) -> Result<StatusCode, ApiError> {
    if auth_session.user.unwrap().id != seller_id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso".to_string()))
    }
    
    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await?;

    let schedule_id = sqlx::query_scalar::<_, i32>(
        r#"
            INSERT INTO schedules (geolocation, address, start_time, end_time, day_of_week)
            VALUES (ST_SetSRID(ST_MakePoint($1, $2), 4326), $3, $4, $5, $6)
            RETURNING id
        "#
    )
        .bind(payload.location.latitude)
        .bind(payload.location.longitude)
        .bind(payload.address)
        .bind(payload.start_time)
        .bind(payload.end_time)
        .bind(payload.day_of_week as i16)
        .fetch_one(&mut *tx)
        .await?;

    sqlx::query(
        r#"
            INSERT INTO seller_schedules (seller_id, schedule_id)
            VALUES ($1, $2)
        "#
    )
        .bind(seller_id)
        .bind(schedule_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(StatusCode::CREATED)
}