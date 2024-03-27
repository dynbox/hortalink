use axum::Extension;
use axum::extract::Path;
use crate::app::auth::AuthSession;

use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn user_avatar(
    Path((user_id, image)): Path<(i32, String)>,
    auth_session: AuthSession,
    Extension(state): Extension<AppState>,
) -> Result<(), ApiError> {
    if user_id != auth_session.user.unwrap().id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso!".to_string()))
    }
    
    let parts: Vec<&str> = image.split(".").collect();
    let path = &format!(
        "{}/{}/{}",
        &state.settings.web.cdn.storage,
        user_id,
        parts.first()
            .ok_or(ApiError::NotFound("Nome do arquivo não encontrado".to_string()))?
    );
    let path = std::path::Path::new(path);

    if !path.exists() {
        return Err(ApiError::NotFound("Arquivo não encontrado".to_string()));
    }

    std::fs::remove_file(path)?;
    
    Ok(())
}