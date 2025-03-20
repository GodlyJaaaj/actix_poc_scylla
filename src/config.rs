use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub session: SessionConfig,
    pub oauth: OAuthConfig,
    pub smtp: SmtpConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub protocol: String,
    pub base_url: String,
    pub port: u16,
    pub env: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SessionConfig {
    pub secret: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OAuthConfig {
    pub google: ProviderConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub email_from: String,
    pub frontend_url: String,
    pub tls_mode: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path = "config.toml";

        if !Path::new(config_path).exists() {
            log::error!("Configuration file '{}' not found", config_path);
            process::exit(1);
        }

        // Read the config file
        let config_content = match fs::read_to_string(config_path) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to read config file {}: {}", config_path, e);
                process::exit(1);
            }
        };

        // Parse the TOML content
        match toml::from_str(&config_content) {
            Ok(config) => config,
            Err(e) => {
                log::error!("Failed to parse config file {}: {}", config_path, e);
                process::exit(1);
            }
        }
    }
}
