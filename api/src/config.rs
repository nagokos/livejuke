use std::{env, str::FromStr};

pub struct Config {
    pub app_env: AppEnv,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

impl Config {
    pub fn from_env() -> Result<Config, anyhow::Error> {
        dotenvy::dotenv().ok();

        Ok(Config {
            app_env: env::var("APP_ENV")?.parse()?,
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            jwt_expiration: env::var("JWT_EXPIRATION")?.parse()?,
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
