mod get;
mod post;

use axum::Router;
use axum::routing::post;

pub fn router() -> Router {
    Router::new()
        .route("/:oauth_type", post(post::oauth))
}