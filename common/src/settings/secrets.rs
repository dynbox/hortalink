use std::env::var;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Secrets {
    pub google: OauthSecret,
    pub facebook: OauthSecret,
    pub linkedin: OauthSecret,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OauthSecret {
    pub client_id: String,
    pub client_secret: String,
}

impl Default for Secrets {
    fn default() -> Self {
        Self {
            google: OauthSecret {
                client_id: var("GOOGLE_CLIENT_ID")
                    .unwrap(),
                client_secret: var("GOOGLE_SECRET")
                    .unwrap()
            },
            facebook: OauthSecret {
                client_id: var("FACEBOOK_CLIENT_ID")
                    .unwrap(),
                client_secret: var("FACEBOOK_SECRET")
                    .unwrap()
            },
            linkedin: OauthSecret {
                client_id: var("LINKEDIN_CLIENT_ID")
                    .unwrap(),
                client_secret: var("LINKEDIN_SECRET")
                    .unwrap()
            },
        }
    }
}