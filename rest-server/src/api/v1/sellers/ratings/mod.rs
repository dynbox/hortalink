mod post;
mod patch;
mod delete;

use axum::Router;
use axum::routing::{patch, post};
use axum_login::permission_required;
use common::entities::UserRole;
use crate::app::auth::AuthGate;


pub fn router() -> Router {
    Router::new()
        .route("/", post(post::rating))
        .route("/:rating_id", patch(patch::rating)
            .delete(delete::rating)
        )
        .layer(permission_required!(AuthGate, UserRole::Customer))
}