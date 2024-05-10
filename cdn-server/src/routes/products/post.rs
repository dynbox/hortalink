use axum::Extension;
use axum::extract::{Multipart, Path};
use app_core::image::ImageManager;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn product_photo(
    Path(product_id): Path<i32>,
    auth_session: AuthSession,
    Extension(state): Extension<AppState>,
    mut multipart: Multipart,
) -> Result<String, ApiError> {
    let user_id = auth_session.user.unwrap().id;

    let product_exists: bool = sqlx::query_scalar(
        r#"
            SELECT EXISTS(
                SELECT 1 FROM seller_products
                WHERE id = $1 AND seller_id = $2
            )
        "#
    )
        .bind(product_id)
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?;

    if !product_exists {
        return Err(ApiError::Unauthorized("Você não pode fazer isso!".to_string()));
    }

    while let Some(field) = multipart.next_field().await? {
        let format = field.content_type()
            .ok_or(ApiError::NotFound("Formato de imagem não encontrado".to_string()))?
            .to_string();

        let path = &format!("{}/products/{}", &state.settings.web.cdn.storage, user_id);
        let path = std::path::Path::new(path);

        if !path.exists() {
            std::fs::create_dir(path)?;
        }

        let data = field.bytes().await?;
        let hash = ImageManager::new(path).create_image(&format, data)
            .await?;

        return Ok(hash);
    }

    Err(ApiError::NotFound("Imagem não encontrada".to_string()))
}