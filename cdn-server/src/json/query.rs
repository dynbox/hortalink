use serde::Deserialize;
use common::entities::ImageSize;

#[derive(Deserialize)]
pub struct ImageSizeQuery {
    pub size: ImageSize,
}