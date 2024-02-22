mod ratings;

use axum::Router;


pub fn router() -> Router {
    Router::new()
        .nest("/ratings", ratings::router())
}
