mod me;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/@me", me::router())
}