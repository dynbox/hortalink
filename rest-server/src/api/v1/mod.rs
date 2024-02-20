mod users;
mod sellers;
mod customers;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/users", users::router())
        .nest("/sellers", sellers::router())
        .nest("/customers", customers::router())
}