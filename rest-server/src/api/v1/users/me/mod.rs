mod notifications;
mod get;

use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .nest("/notifications", notifications::router())
        .route("/", get(get::me))
}