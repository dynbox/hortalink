use garde::Validate;
use serde::{Deserialize, Serialize, Serializer};

use crate::models::customers::CustomerUser;
use crate::models::sellers::SellerUser;
use crate::models::users::{ProtectedUser, ViewerUser};

#[derive(Serialize, Deserialize, Validate)]
pub struct PatchUserMe {
    #[garde(length(min = 3, max = 25))]
    pub avatar: Option<String>,
    #[garde(length(min = 5, max = 64))]
    pub name: Option<String>,
    #[garde(length(min = 11, max = 11))]
    pub phone: Option<String>,
    #[garde(email)]
    pub email: Option<String>
}

#[derive(Serialize)]
pub struct UserMeResponse {
    pub user: ProtectedUser,
    pub infos: Vec<UserType>,
}

pub enum UserType {
    Customer(CustomerUser),
    Seller(SellerUser),
    Viewer(ViewerUser),
}

impl Serialize for UserType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            UserType::Customer(user) => user.serialize(serializer),
            UserType::Seller(user) => user.serialize(serializer),
            UserType::Viewer(user) => user.serialize(serializer),
        }
    }
}