use axum::Router;
use axum::routing::{get, post};
use crate::app::AppState;

mod post;
mod get;
pub mod backend;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(post::login))
        .route("/auth/logout", get(get::logout))
        .route("/auth/sign", post(post::sign))
        .route("/oauth/:oauth_type", get(post::oauth))
        .route("/oauth/callback/:oauth_type", get(get::oauth_callback))
}
