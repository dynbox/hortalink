mod ratings;

use axum::Router;


pub fn router() -> Router {
    Router::new()
        .nest("/:customer_id/ratings", ratings::router())
}