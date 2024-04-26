use std::path::Path;

use axum::{Extension, Json};
use axum_garde::WithValidation;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::users::PatchUserMe;

pub async fn me(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<PatchUserMe>>,
) -> Result<(), ApiError> {
    let login_user = auth_session.user.unwrap().id;
    let mut payload = payload.into_inner();

    if let Some(avatar) = &payload.avatar {
        if !Path::new(&format!("{}/avatars/{login_user}/{avatar}", state.settings.web.cdn.storage)).exists() {
            payload.avatar = None
        }
    };

    sqlx::query(
        r#"
            UPDATE users
            SET
                avatar = COALESCE($1, avatar),
                name = COALESCE($2, name),
                phone = COALESCE($3, phone),
                email = COALESCE($4, email)
            WHERE id = $5;
        "#
    )
        .bind(payload.avatar)
        .bind(payload.name)
        .bind(payload.phone)
        .bind(payload.email)
        .bind(login_user)
        .execute(&state.pool)
        .await
        .unwrap();

    Ok(())
}