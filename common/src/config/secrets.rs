use serde::{Deserialize, Serialize};

use super::env_var;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Secrets {
    pub google: OauthSecret,
    pub facebook: OauthSecret,
    pub linkedin: OauthSecret
}

impl Secrets {
    pub(super) fn new_from_env() -> Self {
        Self {
            google: OauthSecret {
                client_id: env_var("GOOGLE_CLIENT_ID"),
                client_secret: env_var("GOOGLE_CLIENT_SECRET")
            },
            facebook: OauthSecret {
                client_id: env_var("FACEBOOK_CLIENT_ID"),
                client_secret: env_var("FACEBOOK_CLIENT_SECRET")
            },
            linkedin: OauthSecret {
                client_id: env_var("LINKEDIN_CLIENT_ID"),
                client_secret: env_var("LINKEDIN_CLIENT_SECRET")
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OauthSecret {
    pub client_id: String,
    pub client_secret: String,
}
