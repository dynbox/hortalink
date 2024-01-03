use serde::{Deserialize, Serialize};

use super::env_var;

#[derive(Serialize, Deserialize, Clone)]
pub struct Secrets {
    pub jwt: String,
}

impl Secrets {
    pub(super) fn new_from_env() -> Self {
        Self {
            jwt: env_var("JWT_KEY"),
        }
    }

    pub(super) fn echo_env(&self) {
        std::env::set_var("JWT_KEY", &self.jwt)
    }
}

impl Default for Secrets {
    fn default() -> Self {
        Self {
            jwt: String::default(),
        }
    }
}
