use axum::{Extension, Json};

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;
use crate::json::home::Home;
use crate::json::utils::Pagination;
use crate::models::users::PreviewUser;

pub async fn home(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
) -> Result<Json<Home>, ApiError> {
    let user = auth_session.user.as_ref().unwrap();

    if user.roles.contains(&3) {
        let recents = super::most_recent::get::fetch(
            user.id,
            Pagination { page: 1, per_page: 9 },
            &state.pool,
        )
            .await?;

        let more_orders = super::more_orders::get::fetch(
            user.id,
            Pagination { page: 1, per_page: 9 },
            &state.pool,
        )
            .await?;

        let recommendations = sqlx::query_as::<_, PreviewUser>(
            r#"
                SELECT s.user_id, u.avatar as user_avatar, u.name as user_name
                FROM sellers s
                JOIN (
                    SELECT DISTINCT seller_id
                    FROM seller_products
                ) sp ON s.user_id = sp.seller_id
                JOIN users u ON u.id = s.user_id
                ORDER BY RANDOM()
                LIMIT 9;
            "#
        )
            .fetch_all(&state.pool)
            .await?;

        return Ok(Json(Home {
            recents: Some(recents),
            more_orders: Some(more_orders),
            recommendations: Some(recommendations),
        }));
    } else {}

    return Ok(Json(Home {
        recents: None,
        more_orders: None,
        recommendations: None,
    }));
}