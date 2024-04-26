use axum::Router;
use axum::routing::{delete, get, post};
use axum_login::permission_required;

use common::entities::UserRole;

use crate::app::auth::AuthGate;

mod get;
mod delete;
mod post;

pub fn router() -> Router {
    Router::new()
        .route("/:product_id", post(post::product_photo))
        .route("/:product_id/:hash", delete(delete::product_photo))
        .route_layer(permission_required!(AuthGate, UserRole::Seller))
        .route("/:product_id/:hash", get(get::product_photo))
}