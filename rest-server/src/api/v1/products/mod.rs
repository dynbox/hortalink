mod get;

use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get::filter_products))
        .route("/resources", get(get::products))
}