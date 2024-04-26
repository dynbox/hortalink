use axum::Router;

mod avatars;
mod products;

pub fn router() -> Router {
    Router::new()
        .nest("/avatars", avatars::router())
        .nest("/products", products::router())
}