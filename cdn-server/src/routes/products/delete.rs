use axum::Extension;
use axum::extract::Path;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn product_photo(
    Path((product_id, image)): Path<(i32, String)>,
    auth_session: AuthSession,
    Extension(state): Extension<AppState>,
) -> Result<(), ApiError> {
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

    std::fs::remove_file(path)?;

    Ok(())
}