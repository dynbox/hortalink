use axum::{Extension, Json};
use axum::extract::Path;

use crate::app::provider::OAuthProvider;
use crate::json::auth::AuthUrlResponse;
use crate::json::error::ApiError;

pub async fn oauth(
    Extension(providers): Extension<OAuthProvider>,
    Path(oauth_type): Path<String>,
) -> Result<Json<AuthUrlResponse>, ApiError> {
    let ((auth_url, _),) = providers.get_provider(&oauth_type)
        .auth_url();

    let response = AuthUrlResponse {
        auth_url: if oauth_type == "linkedin" {
            auth_url.as_str().replace("+", "%20")
        } else {
            auth_url.as_str().to_string()
        }
    };
    
    Ok(Json(response))
}