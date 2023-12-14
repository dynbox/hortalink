use std::{path::Path, fs, process};

use super::{Settings, webserver::WebServerSettings, redis::RedisSettings, rabbit::RabbitSettings, secrets::Secrets};

const CONFIG_PATH: &str = "application.toml";

impl Settings {
    pub fn new() -> Settings {
        if Path::new(CONFIG_PATH).exists() {
            let config: Settings = Self::read(CONFIG_PATH);
            config.echo_env();

            config
        } else {
            match std::env::var("ENVIROMENT").unwrap_or("PRODUCTION".to_string()).as_str() {
                "PRODUCTION" => Self {
                    webserver: WebServerSettings::new(),
                    redis: RedisSettings::new(),
                    rabbitmq: RabbitSettings::new(),
                    secrets: Secrets::new(),
                },
                _ => {
                    Self::write(CONFIG_PATH);
                    process::exit(1);
                }
            }
        }
    }

    fn read(path: &str) -> Self {
        toml::from_str(&fs::read_to_string(path).unwrap()).expect("Error reading file")
    }

    fn write(path: &str) {
        fs::write(
            path,
            toml::to_string(&Self::default()).unwrap()
        ).expect("Error serializing to TOML");
    }

    fn echo_env(&self) {
        self.webserver.echo_env();
        self.secrets.echo_env();
    }
}
