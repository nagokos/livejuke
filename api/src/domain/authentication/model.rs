use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::domain::{
    authentication::{email::Email, error::AuthenticationError, password::Password},
    id::Id,
    user::model::User,
};

#[derive(Debug, Clone)]
pub struct Authentication {
    pub id: Id<Authentication>,
    pub user_id: Id<User>,
    pub provider: Provider,
    pub uid: String,
    pub password_digest: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NewAuthentication {
    pub provider: Provider,
    pub uid: String,
    pub password_digest: Option<String>,
}

impl NewAuthentication {
    pub fn new(provider: Provider, uid: String, password_digest: Option<String>) -> Self {
        Self {
            provider,
            uid,
            password_digest,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Provider {
    Email,
    Google,
}

impl Provider {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Email => "email",
            Self::Google => "google",
        }
    }
}

impl FromStr for Provider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "email" => Ok(Self::Email),
            "google" => Ok(Self::Google),
            _ => Err(anyhow::anyhow!("unknown provider: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct EmailCredentials {
    pub email: Email,
    pub password: Password,
}

impl EmailCredentials {
    pub fn try_new(email: String, password: String) -> Result<Self, AuthenticationError> {
        Ok(Self {
            email: Email::try_new(email)?,
            password: Password::try_new(password)?,
        })
    }
}
