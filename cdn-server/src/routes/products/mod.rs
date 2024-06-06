use axum::Router;
use axum::routing::{delete, get, post};
use axum_login::permission_required;

use common::entities::UserRole;

use crate::app::auth::AuthGate;

mod get;

pub fn router() -> Router {
    Router::new()
        .route("/:product_id/:hash", get(get::product_photo))
}