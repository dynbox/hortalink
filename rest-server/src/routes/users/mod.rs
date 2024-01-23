mod get;

use axum::Router;
use axum::routing::get;
use axum_login::login_required;
use crate::app::AppState;
use crate::routes::auth::backend::Backend;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get::me))
        .route("/users/:id", get(get::user_info))
        .route_layer(login_required!(Backend))
}