mod notifications;

use axum::Router;


pub fn router() -> Router {
    Router::new()
        .nest("/notifications", notifications::router())
}