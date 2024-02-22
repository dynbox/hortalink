mod users;
mod sellers;
mod customers;
mod auth;

use axum::Router;
use axum_login::{login_required, permission_required};
use common::entities::UserRole;
use crate::app::auth::AuthGate;


pub fn router() -> Router {
    Router::new()
        .merge(protected_router())
        .layer(login_required!(AuthGate))
        .nest("/auth", auth::router())
}

fn protected_router() -> Router {
    Router::new()
        .nest("/sellers", sellers::router())
        .layer(permission_required!(AuthGate, UserRole::Seller))
        .nest("/customers", customers::router())
        .layer(permission_required!(AuthGate, UserRole::Customer))
        .nest("/users", users::router())
}