mod notifications;
mod orders;
mod cart;
mod get;
mod patch;
mod home;
mod store;

use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .nest("/orders", orders::router())
        .nest("/cart", cart::router())
        .nest("/notifications", notifications::router())
        .nest("/home", home::router())
        .nest("/store", store::router())
        .route("/", get(get::me)
            .patch(patch::me)
        )
}