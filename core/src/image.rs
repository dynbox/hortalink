use std::path::Path;
use axum::body::Bytes;

use image::{GenericImageView, ImageError, ImageFormat};
use image::error::{ImageFormatHint, UnsupportedError, UnsupportedErrorKind};
use image::imageops::FilterType;
use image::io::Reader;
use image_hasher::{Hasher, HasherConfig};

use common::entities::ImageSize;

pub struct ImageManager<Q: AsRef<Path>> {
    path: Q,
    hasher: Hasher,
}

impl<Q> ImageManager<Q>
    where Q: AsRef<Path>
{
    pub fn new(path: Q) -> Self {
        Self {
            path,
            hasher: HasherConfig::new().to_hasher(),
        }
    }

    pub async fn get_image(&self, size: ImageSize) -> Result<Vec<u8>, ImageError> {
        let dimensions = size.dimensions();
        let image = Reader::open(&self.path)?
            .with_guessed_format()?
            .decode()?
            .resize(dimensions.0 as u32, dimensions.1 as u32, FilterType::Lanczos3);

        let mut buffer = std::io::Cursor::new(Vec::new());
        image.write_to(&mut buffer, ImageFormat::Png)?;

        Ok(buffer.into_inner())
    }

    pub async fn create_image(&mut self, origin_format: &str, data: Bytes) -> Result<String, ImageError> {
        let format = ImageFormat::from_extension(origin_format)
            .ok_or(ImageError::Unsupported(UnsupportedError::from_format_and_kind(
                ImageFormatHint::Unknown,
                UnsupportedErrorKind::Format(ImageFormatHint::Unknown),
            )))?;

        let mut image = Reader::new(std::io::Cursor::new(data));
        image.set_format(format.clone());

        let image = image.decode()?;
        let dimensions = image.dimensions();
        let image = image.resize(
            dimensions.0 / 3,
            dimensions.1 / 3,
            FilterType::Gaussian,
        );

        let hash = self.hasher.hash_image(&image).to_base64();
        image.save_with_format(
            self.path.as_ref().join(&hash),
            format,
        )?;

        Ok(hash)
    }
}