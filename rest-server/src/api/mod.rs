mod v1;

use axum::Router;
use crate::app::web::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/v1", v1::router())
}