mod ratings;
mod cart;

use axum::Router;
use crate::app::web::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/:customer_id/ratings", ratings::router())
        .nest("/:customer_id/cart", cart::router())
}