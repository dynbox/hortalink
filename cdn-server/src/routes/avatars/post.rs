use std::time::SystemTime;
use axum::Extension;
use axum::extract::{Multipart, Path};
use image::io::Reader;

use crate::app::auth::AuthSession;
use crate::app::server::AppState;
use crate::json::error::ApiError;

pub async fn user_avatar(
    Path(user_id): Path<i32>,
    auth_session: AuthSession,
    Extension(state): Extension<AppState>,
    mut multipart: Multipart,
) -> Result<(), ApiError> {
    /*
    if user_id != auth_session.user.unwrap().id {
        return Err(ApiError::Unauthorized("Você não pode fazer isso!".to_string()))
    }
     */

    while let Some(field) = multipart.next_field().await? {
        let data = field.bytes().await?;

        let image = Reader::new(std::io::Cursor::new(data))
            .with_guessed_format()?
            .decode()?;
        
        let path = &format!("{}/{}", &state.settings.web.cdn.storage, user_id);
        let path = std::path::Path::new(path);

        if !path.exists() {
            std::fs::create_dir(path)?;
        }

        image.save(
            path.join(
                SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .to_string() + ".png"
            )
        )?;
    }

    Ok(())
}