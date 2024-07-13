mod get;

use axum::Router;
use axum_login::permission_required;

use common::entities::UserRole;

use crate::app::auth::AuthGate;

pub fn router() -> Router {
    Router::new()
        .layer(permission_required!(AuthGate, UserRole::Seller))
}