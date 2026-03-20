use axum_extra::extract::cookie::SameSite;

use crate::config::AppEnv;

pub struct CookieConfig {
    pub domain: String,
    pub secure: bool,
    pub same_site: SameSite,
}

impl CookieConfig {
    pub fn from_env(app_env: AppEnv) -> Self {
        match app_env {
            AppEnv::Development => CookieConfig {
                domain: "".to_string(),
                secure: false,
                same_site: SameSite::Lax,
            },
            AppEnv::Production => CookieConfig {
                domain: ".livejuke.app".to_string(),
                secure: true,
                same_site: SameSite::Lax,
            },
        }
    }
}
