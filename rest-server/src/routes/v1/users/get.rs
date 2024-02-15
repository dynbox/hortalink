use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use crate::app::backend::AuthSession;
use crate::app::web::AppState;
use crate::json::users::UserMe;
use crate::models::users::{PublicMe, PublicUser};

pub async fn me(
    auth_session: AuthSession,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let protected_user = auth_session.user.unwrap();
    let public_user: PublicMe = sqlx::query_as(
        r#"
            SELECT name, username, avatar, bio
            FROM users WHERE id = $1
        "#
    )
        .bind(protected_user.id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Json(UserMe {
        id: protected_user.id,
        email: protected_user.email,
        name: public_user.name,
        username: public_user.username,
        avatar: public_user.avatar,
        account_type: protected_user.account_type,
        bio: public_user.bio,
    }).into_response()
}

pub async fn user_info(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user: Option<PublicUser> = sqlx::query_as(
        r#"
            SELECT id, name, username, avatar, bio
            FROM users WHERE id = $1
        "#
    )
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    Json(user).into_response()
}
