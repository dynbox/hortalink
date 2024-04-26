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
        let path = &format!("{}/avatars/{login_user}", state.settings.web.cdn.storage);
        let path = Path::new(path);
        
        if !path.join(avatar).exists() {
            payload.avatar = None
        } else {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.path().file_name().unwrap().to_str().unwrap() != avatar {
                            if let Err(e) = std::fs::remove_file(entry.path()) {
                                eprintln!("Failed to remove file: {}", e);
                            }
                        }
                    }
                }
            }
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