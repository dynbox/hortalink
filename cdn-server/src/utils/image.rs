use std::path::Path;
use std::time::SystemTime;

use axum::body::Bytes;
use image::{GenericImageView, ImageFormat};
use image::imageops::FilterType;
use image::io::Reader;

use common::entities::ImageSize;

use crate::json::error::ApiError;

pub struct ImageManager<Q: AsRef<Path>> {
    path: Q,
}

impl<Q> ImageManager<Q>
    where Q: AsRef<Path>
{
    pub fn new(path: Q) -> Self {
        Self {
            path
        }
    }

    pub async fn get_image(&self, size: ImageSize) -> Result<Vec<u8>, ApiError> {
        let dimensions = size.dimensions();
        let image = Reader::open(&self.path)?
            .with_guessed_format()?
            .decode()?
            .resize(dimensions.0 as u32, dimensions.1 as u32, FilterType::Lanczos3);
        
        let mut buffer = std::io::Cursor::new(Vec::new());
        image.write_to(&mut buffer, ImageFormat::Png)?;
        
        Ok(buffer.into_inner())
    }

    pub async fn create_image(&mut self, origin_format: &str, data: Bytes) -> Result<(), ApiError> {
        let format = ImageFormat::from_extension(origin_format)
            .ok_or(ApiError::NotFound("Formato de imagem não encontrado".to_string()))?;

        let mut image = Reader::new(std::io::Cursor::new(data));
        image.set_format(format);

        let image = image.decode()?;
        let dimensions = image.dimensions();
        let image = image.resize(
            dimensions.0 / 3, 
            dimensions.1 / 3, 
            FilterType::Gaussian
        );

        image.save_with_format(
            self.path.as_ref().join(
                SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .to_string()
            ),
            format,
        )?;

        Ok(())
    }
}