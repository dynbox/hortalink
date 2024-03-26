use axum::Extension;
use axum::extract::Path;

use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn user_avatar(
    Path((user_id, hash)): Path<(i32, String)>,
    Extension(state): Extension<AppState>,
) -> Result<(), ApiError> {
    let path = &format!("{}/{}/{}.png", &state.settings.web.cdn.storage, user_id, hash);
    let path = std::path::Path::new(path);

    if !path.exists() {
        return Err(ApiError::NotFound("Arquivo não encontrado".to_string()));
    }

    std::fs::remove_file(path)?;
    
    Ok(())
}