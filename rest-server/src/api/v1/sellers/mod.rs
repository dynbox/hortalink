use axum::Router;
use axum::routing::get;

mod schedules;
mod products;
mod get;

pub fn router() -> Router {
    Router::new()
        .nest("/:seller_id/schedules", schedules::router())
        .nest("/:seller_id/products", products::router())
        .route("/:seller_id", get(get::seller))
}
