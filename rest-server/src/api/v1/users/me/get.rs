use axum::{Extension, Json};

use crate::app::auth::AuthSession;
use crate::app::web::AppState;
use crate::json::error::ApiError;
use crate::json::users::{UserMeResponse, UserType};
use crate::models::customers::CustomerUser;
use crate::models::sellers::SellerUser;
use crate::models::users::{ProtectedUser, ViewerUser};

pub async fn me(
    Extension(state): Extension<AppState>,
    auth_session: AuthSession,
) -> Result<Json<UserMeResponse>, ApiError> {
    let login_user = auth_session.user.unwrap();
    let mut tx = state.pool.begin().await?;

    let protected_user = sqlx::query_as::<_, ProtectedUser>(
        r#"
            SELECT id, name, avatar, phone, email
            FROM users
            WHERE id = $1
        "#
    )
        .bind(login_user.id)
        .fetch_one(&mut *tx)
        .await?;

    let mut infos = Vec::<UserType>::new();
    for role in login_user.roles {
        if role == 3 {
            let user = sqlx::query_as::<_, CustomerUser>(
                r#"
                    SELECT address FROM customers
                    WHERE user_id = $1
                "#
            )
                .bind(login_user.id)
                .fetch_one(&mut *tx)
                .await?;

            infos.push(UserType::Customer(user))
        } else if role == 2 {
            let user = sqlx::query_as::<_, ViewerUser>(
                r#"
                    SELECT end_time FROM blacklist
                    WHERE user_id = $1
                "#
            )
                .bind(login_user.id)
                .fetch_one(&mut *tx)
                .await?;

            infos.push(UserType::Viewer(user))
        } else if role == 4 {
            let user = sqlx::query_as::<_, SellerUser>(
                r#"
                    SELECT bio FROM sellers
                    WHERE user_id = $1
                "#
            )
                .bind(login_user.id)
                .fetch_one(&mut *tx)
                .await?;
            
            infos.push(UserType::Seller(user))
        }
    }

    tx.commit().await?;

    Ok(Json(UserMeResponse { user: protected_user, infos }))
}