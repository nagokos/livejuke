use std::{env, str::FromStr};

pub struct Config {
    pub app_env: AppEnv,
    pub database_url: String,
    pub redis_url: String,
    pub access_token_secret: String,
    pub access_token_expiration: i64,
    pub refresh_token_expiration: i64,
    pub google_client_id: String,
    pub resend_cooldown_seconds: u8,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,
    pub smtp_tls: String,
    pub max_attempts: u8,
    pub rate_limit: u8,
}

impl Config {
    pub fn from_env() -> Result<Config, anyhow::Error> {
        dotenvy::dotenv().ok();

        Ok(Config {
            app_env: env::var("APP_ENV")?.parse()?,
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")?,
            access_token_secret: env::var("ACCESS_TOKEN_SECRET")?,
            access_token_expiration: env::var("ACCESS_TOKEN_EXPIRATION_SECONDS")?.parse()?,
            refresh_token_expiration: env::var("REFRESH_TOKEN_EXPIRATION_SECONDS")?.parse()?,
            google_client_id: env::var("GOOGLE_CLIENT_ID")?.parse()?,
            resend_cooldown_seconds: env::var("RESEND_COOLDOWN_SECONDS")?.parse()?,
            smtp_host: env::var("SMTP_HOST")?,
            smtp_port: env::var("SMTP_PORT")?.parse()?,
            smtp_username: env::var("SMTP_USERNAME")?,
            smtp_password: env::var("SMTP_PASSWORD")?,
            smtp_from: env::var("SMTP_FROM")?,
            smtp_tls: env::var("SMTP_TLS")?,
            max_attempts: env::var("MAX_ATTEMPTS")?.parse()?,
            rate_limit: env::var("RATE_LIMIT")?.parse()?,
        })
    }
}

pub enum AppEnv {
    Development,
    Production,
}

impl FromStr for AppEnv {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(AppEnv::Development),
            "production" => Ok(AppEnv::Production),
            _ => Err(anyhow::anyhow!("unknown app env {}", s)),
        }
    }
}
