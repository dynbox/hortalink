use axum::Router;

mod schedules;
mod products;

pub fn router() -> Router {
    Router::new()
        .nest("/:seller_id/schedules", schedules::router())
        .nest("/:seller_id/products", products::router())
}
