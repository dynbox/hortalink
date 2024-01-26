use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_login::AuthUser;

use crate::{app::AppState, json::users::UpdateUser, routes::auth::backend::AuthSession};

pub async fn update_user(
    state: State<AppState>,
    auth_session: AuthSession,
    Json(payload): Json<UpdateUser>
) -> impl IntoResponse {
    sqlx::query("UPDATE users SET username = $1, name = $2, phone = $3, address = $4, email = $5, password = $6 WHERE id = $7")
        .bind(payload.username)
        .bind(payload.name)
        .bind(payload.phone)
        .bind(payload.address)
        .bind(payload.email)
        .bind(payload.password)
        .bind(auth_session.user.unwrap().id())
        .execute(&state.pool)
        .await
        .unwrap();

    StatusCode::OK.into_response()
}