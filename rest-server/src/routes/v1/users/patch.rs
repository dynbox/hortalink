use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_garde::WithValidation;
use crate::app::backend::AuthSession;
use crate::app::web::AppState;
use crate::json::users::UpdateUser;

pub async fn update_user(
    state: State<AppState>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<UpdateUser>>
) -> impl IntoResponse {
    let protected_user = auth_session.user.unwrap();

    sqlx::query(
        r#"
            UPDATE users SET
                username = CASE WHEN $1 IS NOT NULL THEN $1 ELSE username END,
                name = CASE WHEN $2 IS NOT NULL THEN $2 ELSE name END,
                phone = CASE WHEN $3 IS NOT NULL THEN $3 ELSE phone END,
                address = CASE WHEN $4 IS NOT NULL THEN $4 ELSE address END,
                email = CASE WHEN $5 IS NOT NULL THEN $5 ELSE email END,
                password = CASE WHEN $6 IS NOT NULL THEN $6 ELSE password END
            WHERE id = $7
        "#
    )
        .bind(&payload.username)
        .bind(&payload.name)
        .bind(&payload.phone)
        .bind(&payload.address)
        .bind(&payload.email)
        .bind(&protected_user.id)
        .execute(&state.pool)
        .await
        .unwrap();

    StatusCode::OK.into_response()
}