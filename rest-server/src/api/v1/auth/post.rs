use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum_garde::WithValidation;
use axum_login::tower_sessions::Session;

use app_core::image::ImageManager;
use common::entities::UserRole;
use common::settings::Protocol;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::auth::{Credentials, LoginCreds, SignCreds};
use crate::json::error::ApiError;
use crate::models::users::LoginUser;

pub async fn login(
    mut auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<LoginCreds>>,
) -> Result<(), ApiError> {
    let user = auth_session.authenticate(
        Credentials::Password(payload.into_inner())
    )
        .await?
        .ok_or(ApiError::NotFound("Usuário não encontrado".to_string()))?;

    auth_session.login(&user)
        .await?;
    Ok(())
}

pub async fn sign_in(
    Extension(state): Extension<AppState>,
    session: Session,
    mut auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<SignCreds>>,
) -> Result<Redirect, ApiError> {
    let payload = payload.into_inner();
    let mut tx = state.pool.begin().await?;

    let user = sqlx::query_as::<_, LoginUser>(
        r#"
            SELECT
                id, password,
                roles, access_token
            FROM USERS
            WHERE email = $1
        "#
    )
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await?;

    if let Some(user) = user {
        auth_session.login(&user).await?;
        return Ok(Redirect::to(&format!("{}", state.settings.web.client.protocol_url())));
    }

    let user = if let Ok(Some(oauth_token)) = session.remove::<String>("oauth.token").await {
        sqlx::query_as(
            r#"
                INSERT INTO users (email, name, access_token, roles)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT(email) DO UPDATE
                SET access_token = excluded.access_token
                RETURNING
                    id, password,
                    roles, access_token
            "#,
        )
            .bind(payload.email)
            .bind(payload.name)
            .bind(oauth_token)
            .bind(vec![payload.role.clone() as i16])
            .fetch_one(&mut *tx)
            .await?
    } else {
        sqlx::query_as::<_, LoginUser>(
            r#"
                INSERT INTO users (name, email, roles, password)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, password, roles, access_token
            "#
        )
            .bind(payload.name)
            .bind(payload.email)
            .bind(vec![payload.role.clone() as i16])
            .bind(password_auth::generate_hash(&payload.password.unwrap()))
            .fetch_one(&mut *tx)
            .await?
    };

    if payload.role == UserRole::Seller {
        sqlx::query(
            r#"
                INSERT INTO sellers (user_id) VALUES ($1)
            "#
        )
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
    } else {
        sqlx::query(
            r#"
                INSERT INTO customers (user_id) VALUES ($1)
            "#
        )
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
    }

    if let Some(avatar) = payload.avatar {
        let decoded_bytes = base64::Engine::decode(&base64::prelude::BASE64_STANDARD, &avatar)
            .map_err(|e| ApiError::Custom(StatusCode::INTERNAL_SERVER_ERROR, format!("Falha ao decodificar imagem: {e}")))?;
        let path = &format!("{}/avatars/{}", &state.settings.web.cdn.storage, user.id);
        let path = std::path::Path::new(path);

        if !path.exists() {
            std::fs::create_dir(path)
                .map_err(|e| ApiError::Custom(StatusCode::INTERNAL_SERVER_ERROR, format!("Falha ao criar repositório: {e}")))?;
        }

        let hash = ImageManager::new(path).load(&decoded_bytes).await?;

        sqlx::query(
            r#"
                UPDATE users SET avatar = $1 WHERE id = $2
            "#
        )
            .bind(hash)
            .bind(user.id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    auth_session.login(&user).await?;
    Ok(Redirect::to(&format!("{}", state.settings.web.client.protocol_url())))
}