use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use crate::app::provider::AuthProviders;
use crate::json::auth::AuthUrlResponse;

pub async fn oauth(
    Extension(providers): Extension<AuthProviders>,
    Path(oauth_type): Path<String>,
) -> impl IntoResponse {
    let (auth_url, _) = providers.get_provider(&oauth_type)
        .unwrap()
        .auth_url();

    /*
    session
        .insert("oauth.csrf-state", csrf.secret())
        .await
        .expect("Serialization should not fail.");
     */

    let response = AuthUrlResponse {
        auth_url: if oauth_type == "linkedin" {
            auth_url.as_str().replace("+", "%20")
        } else {
            auth_url.as_str().to_string()
        }
    };

    Json(&response).into_response()
}