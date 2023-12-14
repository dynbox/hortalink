use serde::{Deserialize, Serialize};

use self::{webserver::WebServerSettings, redis::RedisSettings, rabbit::RabbitSettings, secrets::Secrets};

pub mod settings;
pub mod database;
pub mod redis;
pub mod webserver;
pub mod secrets;
pub mod rabbit;


#[derive(Serialize, Deserialize, Default)]
pub struct Settings {
    pub webserver: WebServerSettings,
    pub redis: RedisSettings,
    pub rabbitmq: RabbitSettings,
    pub secrets: Secrets,
}