use axum::Router;
use crate::app::web::AppState;

mod auth;
mod users;
mod oauth;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/oauth", oauth::router())
}
