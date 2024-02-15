use serde::Deserialize;

#[derive(Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32
}