use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub port: u16,
    pub env: String,
    pub database_url: String,
    pub redis_url: String,
    pub session_secret: String,
    pub oauth: OAuthConfig,
    pub smtp: SmtpConfig,
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
        dotenv().ok();

        Config {
            base_url: env::var("BASE_URL").unwrap_or_else(|_| "http://localhost".to_string()),
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
            smtp: SmtpConfig {
                server: env::var("SMTP_SERVER").unwrap_or_else(|_| "sandbox.smtp.mailtrap.io".to_string()),
                port: env::var("SMTP_PORT")
                    .unwrap_or_else(|_| "2525".to_string())
                    .parse()
                    .unwrap_or(2525),
                username: env::var("SMTP_USERNAME").unwrap_or_else(|_| "your_username".to_string()),
                password: env::var("SMTP_PASSWORD").unwrap_or_else(|_| "your_password".to_string()),
                email_from: env::var("SMTP_EMAIL_FROM")
                    .unwrap_or_else(|_| "noreply@example.com".to_string()),
                frontend_url: env::var("SMTP_FRONTEND_URL")
                    .unwrap_or_else(|_| "http://localhost:8080".to_string()),
                tls_mode: env::var("SMTP_TLS_MODE").unwrap_or_else(|_| "opportunistic".to_string()),
            },
        }
    }
}
