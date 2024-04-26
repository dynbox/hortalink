use axum::Router;
use axum::routing::{delete, get, post};
use axum_login::login_required;
use crate::app::auth::AuthGate;

mod post;
mod get;
mod delete;

pub fn router() -> Router {
    Router::new()
        .route("/:user_id", post(post::user_avatar))
        .route("/:user_id/:hash", delete(delete::user_avatar))
        .route_layer(login_required!(AuthGate))
        .route("/:user_id/:hash", get(get::user_avatar))
}