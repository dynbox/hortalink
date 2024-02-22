mod users;
mod sellers;
mod customers;
mod auth;

use axum::Router;
use axum_login::{login_required, permission_required};
use common::entities::UserRole;
use crate::app::auth::AuthGate;
use crate::app::web::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/sellers", sellers::router())
        .route_layer(permission_required!(AuthGate, UserRole::Seller))
        .nest("/customers", customers::router())
        .route_layer(permission_required!(AuthGate, UserRole::Customer))
        .nest("/users", users::router())
        .route_layer(login_required!(AuthGate))
        .nest("/auth", auth::router())
}