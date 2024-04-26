use axum::Extension;
use axum::extract::{Multipart, Path};

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::utils::image::ImageManager;

pub async fn user_avatar(
    Path(user_id): Path<i32>,
    auth_session: AuthSession,
    Extension(state): Extension<AppState>,
    mut multipart: Multipart,
) -> Result<String, ApiError> {
    if user_id != auth_session.user.unwrap().id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso!".to_string()));
    }

    while let Some(field) = multipart.next_field().await? {
        let format = field.content_type()
            .ok_or(ApiError::NotFound("Formato de imagem não encontrado".to_string()))?
            .to_string();

        let path = &format!("{}/avatars/{}", &state.settings.web.cdn.storage, user_id);
        let path = std::path::Path::new(path);

        if !path.exists() {
            std::fs::create_dir(path)?;
        }

        let data = field.bytes().await?;
        let hash = ImageManager::new(path).create_image(&format.split("/").last().unwrap(), data)
            .await?;

        return Ok(hash);
    }

    Err(ApiError::NotFound("Imagem não encontrada".to_string()))
}