mod avatars;
mod products;

use axum::Router;
use axum_login::login_required;
use crate::app::auth::AuthGate;

pub fn router() -> Router {
    Router::new()
        .nest("/avatars", avatars::router())
        .nest("/products", products::router())
}