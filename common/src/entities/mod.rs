use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(TryFromPrimitive, IntoPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i16)]
pub enum StarRating {
    VeryBad = 1,
    Bad = 2,
    Medium = 3,
    Good = 4,
    VeryGood = 5,
}

#[derive(TryFromPrimitive, IntoPrimitive, Deserialize_repr, Serialize_repr, Eq, PartialEq)]
#[derive(Clone, Debug)]
#[repr(i16)]
pub enum UserRole {
    Verified = 1,
    Viewer = 2,
    Customer = 3,
    Seller = 4,
    Administrator = 5,
}

#[derive(TryFromPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i16)]
pub enum NotificationType {
    Info = 1,
    Success = 2,
    Error = 3,
}

#[derive(TryFromPrimitive, IntoPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i16)]
pub enum WeekDay {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6
}

#[derive(IntoPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i16)]
pub enum ImageSize {
    Size16 = 16,
    Size32 = 32,
    Size64 = 64,
    Size128 = 128,
    Size256 = 256,
    Size512 = 512,
    Size1024 = 1024,
    Size2048 = 2048,
    Size4096 = 4096,
}

impl ImageSize {
    pub fn dimensions(self) -> (i16, i16) {
        let size = self.into();
        (size, size)
    }
}