use axum::{Extension, Json};
use axum::response::IntoResponse;
use common::entities::UserRole;
use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::users::{UserMeResponse, UserType};
use crate::models::users::{CustomerUser, ProtectedUser, ViewerUser};

pub async fn me(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    let login_user = auth_session.user.unwrap();

    let mut tx = state.pool.begin().await.unwrap();

    let protected_user = sqlx::query_as::<_, ProtectedUser>(
        r#"
            SELECT id, name, avatar, phone, RTRIM(username) AS username, email
            FROM users
            WHERE id = $1
        "#
    )
        .bind(login_user.id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    let user_info = match login_user.role {
        UserRole::Customer => {
            let user = sqlx::query_as::<_, CustomerUser>(
                r#"
                    SELECT address FROM customers
                    WHERE user_id = $1
                "#
            )
                .bind(login_user.id)
                .fetch_one(&mut *tx)
                .await
                .unwrap();

            Some(UserType::Customer(user))
        },
        UserRole::Viewer => {
            let user = sqlx::query_as::<_, ViewerUser>(
                r#"
                    SELECT end_time FROM blacklist
                    WHERE user_id = $1
                "#
            )
                .bind(login_user.id)
                .fetch_one(&mut *tx)
                .await
                .unwrap();

            Some(UserType::Viewer(user))
        },
        _ => None
    };

    tx.commit().await;

    Json(UserMeResponse { user: protected_user, info: user_info })
}