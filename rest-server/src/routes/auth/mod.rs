use axum::Router;
use axum::routing::{get, post};
use axum_login::login_required;
use crate::app::AppState;
use crate::routes::auth::backend::Backend;

mod post;
mod get;
pub mod backend;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/logout", get(get::logout))
        .route_layer(login_required!(Backend))
        .route("/auth/login", post(post::login))
        .route("/auth/sign", post(post::sign))
        .route("/oauth/:oauth_type", post(post::oauth))
        .route("/oauth/callback/:oauth_type", get(get::oauth_callback))
}
