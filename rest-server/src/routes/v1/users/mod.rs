mod get;
mod patch;

use axum::Router;
use axum::routing::get;
use axum_login::login_required;
use crate::app::backend::Backend;
use crate::app::web::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get::me)
            .patch(patch::update_user)
        )
        .route_layer(login_required!(Backend))
        .route("/:id", get(get::user_info))
}