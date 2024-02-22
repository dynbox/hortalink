mod notifications;

use axum::Router;
use crate::app::web::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/notifications", notifications::router())
}