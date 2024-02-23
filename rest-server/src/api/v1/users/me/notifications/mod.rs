use axum::Router;
use axum::routing::{get, patch};

mod get;
mod patch;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get::notifications))
        .route("/:notification_id", patch(patch::read))
}