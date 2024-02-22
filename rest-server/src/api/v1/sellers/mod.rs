use axum::Router;
use crate::app::web::AppState;

mod schedules;
mod products;
mod ratings;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/:seller_id/schedules", schedules::router())
        .nest("/:seller_id/products", products::router())
        .nest("/:seller_id/ratings", ratings::router())
}
