mod post;
mod get;
mod delete;

use axum::Router;
use axum::routing::{get, post};
use axum_login::login_required;
use crate::app::auth::AuthGate;

pub fn router() -> Router {
    Router::new()
        .route("/:user_id", post(post::user_avatar))
        .route("/:user_id/:hash", 
               get(get::user_avatar)
                   .delete(delete::user_avatar)
        )
        //.route_layer(login_required!(AuthGate))
}