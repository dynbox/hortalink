use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::error_message;
use crate::json::schedules::{ScheduleParams, UpdateSchedulePayload};

pub async fn schedule(
    Extension(state): Extension<AppState>,
    Path(params): Path<ScheduleParams>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<UpdateSchedulePayload>>,
) -> impl IntoResponse {
    let login_user = auth_session.user.unwrap();
    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await.unwrap();

    let is_author = sqlx::query_scalar::<_, bool>(
        r#"
            SELECT EXISTS (
                SELECT 1 FROM seller_schedules
                WHERE seller_id = $1 AND schedule_id = $2
            )
        "#
    )
        .bind(login_user.id)
        .bind(params.schedule_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

    if !is_author {
        tx.rollback().await.unwrap();

        return error_message(
            StatusCode::UNAUTHORIZED,
            "Você não é o dono da notificação ou a notificação não existe",
        );
    }

    sqlx::query(r#"
        UPDATE schedules
        SET
            geolocation = COALESCE($1::geometry, geolocation),
            address = COALESCE($2, address),
            start_time = COALESCE($3, start_time),
            end_time = COALESCE($4, end_time),
            day_of_week = COALESCE($5, day_of_week)
        WHERE id = $6
    "#)
        .bind(payload.location
            .map(|loc| {
                format!("ST_SetSRID(ST_MakePoint({}, {}),  4326)", loc.latitude, loc.longitude)
            }))
        .bind(payload.address)
        .bind(payload.start_time)
        .bind(payload.end_time)
        .bind(payload.day_of_week
            .map(|day| day as i16))
        .bind(params.schedule_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();
    StatusCode::OK.into_response()
}