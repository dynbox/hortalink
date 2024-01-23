use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use crate::app::AppState;
use crate::models::users::User;
use crate::routes::auth::backend::AuthSession;

pub async fn me(
    auth_session: AuthSession,
) -> impl IntoResponse {
    Json(auth_session.user.unwrap()).into_response()
}

pub async fn user_info(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    Json(user).into_response()
}
