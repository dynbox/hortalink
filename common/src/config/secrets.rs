use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    pub jwt: String,
}

impl Secrets {
    pub fn new() -> Self {
        Self {
            jwt: env::var("JWT_KEY").unwrap(),
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
