mod ratings;
mod get;
mod post;
mod patch;
mod delete;

use axum::Router;
use axum::routing::{delete, get, post};
use axum_login::permission_required;
use common::entities::UserRole;
use crate::app::auth::AuthGate;


pub fn router() -> Router {
    Router::new()
        .route("/:product_id", delete(delete::product)
            .patch(patch::product)
        )
        .route("/", post(post::product))
        .route_layer(permission_required!(AuthGate, UserRole::Seller, UserRole::Verified))
        .route("/:product_id", get(get::product))
        .nest("/ratings", ratings::router())
}
