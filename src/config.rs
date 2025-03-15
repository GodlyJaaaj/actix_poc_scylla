use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub env: String,
    pub database_url: String,
    pub redis_url: String,
    pub session_secret: String,
    pub oauth: OAuthConfig,
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

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Config {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            env: env::var("ENV").unwrap_or_else(|_| "dev".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_default(),
            redis_url: env::var("REDIS_URL").unwrap_or_default(),
            session_secret: env::var("SESSION_SECRET").unwrap_or_default(),
            oauth: OAuthConfig {
                google: ProviderConfig {
                    client_id: env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
                    client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
                    redirect_url: env::var("GOOGLE_REDIRECT_URL")
                        .unwrap_or_else(|_| "".to_string()),
                },
            },
        }
    }
}
