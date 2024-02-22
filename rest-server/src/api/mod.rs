mod v1;

use axum::Router;


pub fn router() -> Router {
    Router::new()
        .nest("/v1", v1::router())
}