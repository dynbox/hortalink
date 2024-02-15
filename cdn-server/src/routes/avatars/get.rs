use std::io::Cursor;
use std::path::PathBuf;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::json::query::Resolution;
use crate::routes::STORAGE_PATH;

pub async fn get_user_avatar(
    Path(user_id): Path<String>,
    Path(hash): Path<String>,
    Query(resolution): Query<Resolution>
) -> impl IntoResponse {
    let storage_path = PathBuf::from(STORAGE_PATH);
    let image_path = storage_path.join(&user_id).join(&hash);

    match image::open(&image_path) {
        Ok(img) => {
            let resized_img = img.resize(
                resolution.width,
                resolution.height,
                image::imageops::FilterType::Nearest
            );
            let mut bytes = vec![];

            if let Err(e) = resized_img.write_to(
                &mut Cursor::new(&mut bytes),
                image::ImageOutputFormat::Png
            ) {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to resize image: {}", e))
                    .into_response();
            }

            bytes.into_response()
        },
        Err(e) => (StatusCode::NOT_FOUND, format!("Failed to open image: {}", e))
            .into_response(),
    }
}
