use axum::Extension;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use tokio::io::AsyncReadExt;

use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn user_avatar(
    Path((user_id, hash)): Path<(i32, String)>,
    Extension(state): Extension<AppState>,
) -> Result<Response, ApiError> {
    let path = &format!("{}/{}/{}.png", &state.settings.web.cdn.storage, user_id, hash);
    let path = std::path::Path::new(path);

    if !path.exists() {
        return Err(ApiError::NotFound("Arquivo não encontrado".to_string()));
    }

    let mut stream = tokio::fs::File::open(path).await?;
    let mut buf = Vec::new();

    stream.read_to_end(&mut buf).await?;
    
    Ok(buf.into_response())
}