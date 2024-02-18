use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StarRating {
    VeryBad = 1,
    Bad = 2,
    Medium = 3,
    Good = 4,
    VeryGood = 5
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserRole {
    Viewer = 1,
    Customer = 2,
    Seller = 3,
    Administrator = 4
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NotificationType {
    Info = 1,
    Success = 2,
    Error = 3
}