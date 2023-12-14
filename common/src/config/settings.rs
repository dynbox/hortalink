use std::{fs, path::Path, process};

use log::{error, info, warn};

use super::{
    rabbit::RabbitSettings, redis::RedisSettings, secrets::Secrets, webserver::WebServerSettings,
    Settings,
};

impl Settings {
    pub fn new(path: &str) -> Settings {
        let path = Path::new(path);

        if path.exists() {
            info!(
                "Reading settings from '{}' file...",
                path.display().to_string()
            );

            let config = Self::read(path);
            config.echo_env();

            config
        } else {
            let enviroment = std::env::var("ENVIRONMENT").unwrap_or("PRODUCTION".to_string());

            match enviroment.as_str() {
                "PRODUCTION" | "STAGE" => {
                    info!("Creating settings for {enviroment} environment...");

                    Self::new_from_env()
                }
                "DEVELOPMENT" => {
                    Self::write(path);
                    warn!("A new settings file has been created at '{}'. Please edit the variables. Exiting...", path.display().to_string());

                    process::exit(1);
                }
                _ => {
                    error!("Environment not recognized. Please edit '.cargo/config.toml' enviroment variable. Exiting...");
                    process::exit(1);
                }
            }
        }
    }

    fn read(path: &Path) -> Self {
        toml::from_str(&fs::read_to_string(path).unwrap()).expect("Error reading file")
    }

    fn write(path: &Path) {
        fs::write(path, toml::to_string(&Self::default()).unwrap())
            .expect("Error serializing to TOML");
    }

    fn new_from_env() -> Self {
        Self {
            webserver: WebServerSettings::new_from_env(),
            redis: RedisSettings::new_from_env(),
            rabbitmq: RabbitSettings::new_from_env(),
            secrets: Secrets::new_from_env(),
        }
    }

    fn echo_env(&self) {
        self.webserver.echo_env();
        self.secrets.echo_env();
    }
}
