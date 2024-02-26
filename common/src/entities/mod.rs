use derivative::Derivative;
use num_enum::TryFromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(TryFromPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i16)]
pub enum StarRating {
    VeryBad = 1,
    Bad = 2,
    Medium = 3,
    Good = 4,
    VeryGood = 5
}

#[derive(TryFromPrimitive, Deserialize_repr, Serialize_repr, Derivative)]
#[derive(Clone, Debug)]
#[derivative(PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum UserRole {
    Viewer = 1,
    Customer = 2,
    Seller = 3,
    Administrator = 4
}

#[derive(TryFromPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i16)]
pub enum NotificationType {
    Info = 1,
    Success = 2,
    Error = 3
}

#[derive(TryFromPrimitive, Deserialize_repr, Serialize_repr)]
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