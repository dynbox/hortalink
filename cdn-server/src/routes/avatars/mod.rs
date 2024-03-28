use axum::Router;
use axum::routing::{get, post};

mod post;
mod get;
mod delete;

pub fn router() -> Router {
    Router::new()
        .route("/:user_id", post(post::user_avatar))
        .route("/:user_id/:hash",
               get(get::user_avatar)
                   .delete(delete::user_avatar),
        )
}