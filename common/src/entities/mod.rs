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

#[derive(TryFromPrimitive, IntoPrimitive, Deserialize_repr, Serialize_repr)]
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
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}