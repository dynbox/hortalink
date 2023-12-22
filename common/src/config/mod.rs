use serde::{Deserialize, Serialize};
use crate::config::cdn_server::CdnServer;

use self::{
    rabbit::RabbitSettings, redis::RedisSettings, secrets::Secrets, webserver::WebServerSettings,
};

pub mod database;
pub mod rabbit;
pub mod redis;
pub mod secrets;
pub mod settings;
pub mod webserver;
pub mod cdn_server;

#[derive(Serialize, Deserialize, Default)]
pub struct Settings {
    pub webserver: WebServerSettings,
    #[serde(rename="cdn-server")]
    pub cdn_server: CdnServer,
    pub redis: RedisSettings,
    pub rabbitmq: RabbitSettings,
    pub secrets: Secrets,
}

fn env_var<K: AsRef<std::ffi::OsStr>>(var_name: K) -> String {
    std::env::var(&var_name).unwrap_or_else(|_| {
        log::error!(
            "Environment variable '{}' not present.",
            var_name.as_ref().to_str().unwrap()
        );
        std::process::exit(1);
    })
}
