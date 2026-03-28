use std::{env, str::FromStr};

pub struct Config {
    pub app_env: AppEnv,
    pub database_url: String,
    pub access_token_secret: String,
    pub access_token_expiration: i64,
    pub refresh_token_expiration: i64,
    pub google_client_id: String,
}

impl Config {
    pub fn from_env() -> Result<Config, anyhow::Error> {
        dotenvy::dotenv().ok();

        Ok(Config {
            app_env: env::var("APP_ENV")?.parse()?,
            database_url: env::var("DATABASE_URL")?,
            access_token_secret: env::var("ACCESS_TOKEN_SECRET")?,
            access_token_expiration: env::var("ACCESS_TOKEN_EXPIRATION")?.parse()?,
            refresh_token_expiration: env::var("REFRESH_TOKEN_EXPIRATION")?.parse()?,
            google_client_id: env::var("GOOGLE_CLIENT_ID")?.parse()?,
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
