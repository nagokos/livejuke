use std::{env, str::FromStr};

pub struct Config {
    pub app_env: AppEnv,
    pub database_url: String,
    pub redis_url: String,
    pub access_token_secret: String,
    pub access_token_exp_secs: u64,
    pub refresh_token_exp_secs: u64,
    pub google_client_id: String,
    pub resend_cooldown_secs: u8,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,
    pub smtp_tls: String,
    pub verification_code_exp_secs: u64,
    pub max_attempts: i64,
    pub max_attempts_ttl_secs: u64,
    pub rate_limit: i64,
    pub rate_limit_ttl_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Config, anyhow::Error> {
        dotenvy::dotenv().ok();

        Ok(Config {
            app_env: env::var("APP_ENV")?.parse()?,
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")?,
            access_token_secret: env::var("ACCESS_TOKEN_SECRET")?,
            access_token_exp_secs: env::var("ACCESS_TOKEN_EXPIRATION_SECONDS")?.parse()?,
            refresh_token_exp_secs: env::var("REFRESH_TOKEN_EXPIRATION_SECONDS")?.parse()?,
            google_client_id: env::var("GOOGLE_CLIENT_ID")?.parse()?,
            resend_cooldown_secs: env::var("RESEND_COOLDOWN_SECONDS")?.parse()?,
            smtp_host: env::var("SMTP_HOST")?,
            smtp_port: env::var("SMTP_PORT")?.parse()?,
            smtp_username: env::var("SMTP_USERNAME")?,
            smtp_password: env::var("SMTP_PASSWORD")?,
            smtp_from: env::var("SMTP_FROM")?,
            smtp_tls: env::var("SMTP_TLS")?,
            verification_code_exp_secs: env::var("VERIFICATION_CODE_EXP_SECS")?.parse()?,
            max_attempts: env::var("MAX_ATTEMPTS")?.parse()?,
            max_attempts_ttl_secs: env::var("MAX_ATTEMPTS_TTL_SECONDS")?.parse()?,
            rate_limit: env::var("RATE_LIMIT")?.parse()?,
            rate_limit_ttl_secs: env::var("RATE_LIMIT_TTL_SECONDS")?.parse()?,
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
