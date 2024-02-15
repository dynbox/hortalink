use axum::Router;
use axum::routing::{get, post};
use crate::app::web::AppState;

mod get;
mod post;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:oauth_type", post(post::oauth))
        .route("/callback/:oauth_type", get(get::oauth_callback))
}