use axum::Router;
use axum::routing::get;

mod schedules;
mod products;
mod ratings;
mod get;

pub fn router() -> Router {
    Router::new()
        .nest("/:seller_id/schedules", schedules::router())
        .nest("/:seller_id/products", products::router())
        .nest("/:seller_id/ratings", ratings::router())
        .route("/:seller_id", get(get::seller))
}
