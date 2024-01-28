mod get;
mod patch;

use axum::Router;
use axum::routing::{get, patch};
use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get::me))
        .route("/users/me", patch(patch::update_user))
        .route("/users/:id", get(get::user_info))
}