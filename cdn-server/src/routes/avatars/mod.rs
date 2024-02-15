use axum::Router;
use axum::routing::{get, put};

mod get;
mod put;

pub fn router() -> Router {
    Router::new()
        .route("/avatars/:user_id/:hash", get(get::get_user_avatar))
        .route("/avatars/:user_id", put(put::put_avatar))
}