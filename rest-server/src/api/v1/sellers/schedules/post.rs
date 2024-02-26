use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::schedules::CreateSchedulePayload;

pub async fn schedule(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<CreateSchedulePayload>>,
) -> impl IntoResponse {
    let user_id = auth_session.user.unwrap().id;
    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await.unwrap();

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
        .await
        .unwrap();

    sqlx::query(
        r#"
            INSERT INTO seller_schedules (seller_id, schedule_id)
            VALUES ($1, $2)
        "#
    )
        .bind(user_id)
        .bind(schedule_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    tx.commit().await;

    return StatusCode::CREATED.into_response();
}