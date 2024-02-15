use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use axum::extract::Multipart;
use axum::response::IntoResponse;
use http::StatusCode;
use image::io::Reader;
use crate::routes::STORAGE_PATH;

pub async fn put_avatar(
    mut parts: Multipart
) -> impl IntoResponse {
    while let Ok(field) = parts.next_field().await {
        let mut field = match field {
            Some(field) => field,
            None => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        };

        let content_type = field.content_type().unwrap();
        let field_name = field.name().unwrap();

        if field_name == "avatar" && content_type.starts_with("image/") {
            let mut buffer = Vec::new();

            while let Ok(Some(chunk)) = field.chunk().await {
                buffer.extend_from_slice(&chunk);
            }

            let img = match Reader::new(Cursor::new(buffer))
                .with_guessed_format() {
                Ok(img) => img.decode().unwrap(),
                Err(_) => return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response(),
            };

            let storage_path = PathBuf::from(STORAGE_PATH);

            if !storage_path.exists() {
                fs::create_dir_all(&storage_path);
            }

            let file_path = storage_path.join(format!("{}.png", "test"));

            if let Err(_) = img.save(file_path) {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }

            return StatusCode::OK.into_response();
        }
    }

    StatusCode::BAD_REQUEST.into_response()
}