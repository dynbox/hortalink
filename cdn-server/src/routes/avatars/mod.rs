use axum::Router;
use axum::routing::{delete, get};
use axum_login::login_required;

use crate::app::auth::AuthGate;

mod get;

pub fn router() -> Router {
    Router::new()
        .route("/:user_id/:hash", get(get::user_avatar))
}