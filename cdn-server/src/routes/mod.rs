mod avatars;
mod attachments;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/avatars", avatars::router())
        .nest("/attachments", attachments::router())
}