mod users;
mod sellers;
mod customers;
mod auth;
mod products;

use axum::Router;
use axum_login::{login_required};
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
        .nest("/customers", customers::router())
        .nest("/users", users::router())
}