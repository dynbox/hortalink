mod get;
mod post;

use axum::Router;
use axum::routing::{get, post};

pub fn router() -> Router {
    Router::new()
        .route("/:oauth_type", post(post::oauth))
        .route("/:oauth_type/callback", get(get::oauth_callback))
}