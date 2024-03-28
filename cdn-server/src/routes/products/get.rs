use axum::Extension;
use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Response};

use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::query::ImageSizeQuery;
use crate::utils::image::ImageManager;

pub async fn product_photo(
    Path((product_id, image)): Path<(i32, String)>,
    Query(query): Query<ImageSizeQuery>,
    Extension(state): Extension<AppState>,
) -> Result<Response, ApiError> {
    let parts: Vec<&str> = image.split(".").collect();
    let path = &format!(
        "{}/products/{}/{}",
        &state.settings.web.cdn.storage,
        product_id,
        parts.first()
            .ok_or(ApiError::NotFound("Nome do arquivo não encontrado".to_string()))?
    );

    let path = std::path::Path::new(path);

    if !path.exists() {
        return Err(ApiError::NotFound("Arquivo não encontrado".to_string()));
    }

    path.with_extension(
        parts.last()
            .ok_or(ApiError::NotFound("Extensão do arquivo não encontrado".to_string()))?
    );

    ImageManager::new(path).get_image(query.size)
        .await
        .map(|op| op.into_response())
}