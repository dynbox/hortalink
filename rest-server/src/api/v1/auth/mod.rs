mod get;
mod post;

use axum::Router;
use axum::routing::{get, post};
use axum_login::login_required;
use crate::app::auth::AuthGate;
use crate::app::web::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/logout", get(get::logout))
        .route_layer(login_required!(AuthGate))
        .route("/login", post(post::login))
        .route("/sign-in", post(post::sign_in))
}