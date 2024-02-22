mod ratings;

use axum::Router;
use crate::app::web::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/ratings", ratings::router())
}
