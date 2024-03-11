mod ratings;
mod cart;

use axum::Router;


pub fn router() -> Router {
    Router::new()
        .nest("/:customer_id/ratings", ratings::router())
        .nest("/:customer_id/cart", cart::router())
}